macro_rules! deps {
    () => {
        Endian!();
        UnalignedU16Slice!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl UnalignedU16Slice { # [doc = " Safety: ptr must be valid for reading 2*len bytes"] # [inline (always)] pub unsafe fn new (ptr : * const u8 , len : usize) -> UnalignedU16Slice { UnalignedU16Slice { ptr , len } } # [inline (always)] pub fn trim_last (& mut self) { assert ! (self . len > 0) ; self . len -= 1 ; } # [inline (always)] pub fn at (& self , i : usize) -> u16 { use core :: mem :: MaybeUninit ; assert ! (i < self . len) ; unsafe { let mut u : MaybeUninit < u16 > = MaybeUninit :: uninit () ; :: core :: ptr :: copy_nonoverlapping (self . ptr . add (i * 2) , u . as_mut_ptr () as * mut u8 , 2) ; u . assume_init () } } # [cfg (feature = "simd-accel")] # [inline (always)] pub fn simd_at (& self , i : usize) -> u16x8 { assert ! (i + SIMD_STRIDE_SIZE / 2 <= self . len) ; let byte_index = i * 2 ; unsafe { to_u16_lanes (load16_unaligned (self . ptr . add (byte_index))) } } # [inline (always)] pub fn len (& self) -> usize { self . len } # [inline (always)] pub fn tail (& self , from : usize) -> UnalignedU16Slice { assert ! (from <= self . len) ; unsafe { UnalignedU16Slice :: new (self . ptr . add (from * 2) , self . len - from) } } # [cfg (feature = "simd-accel")] # [inline (always)] pub fn copy_bmp_to < E : Endian > (& self , other : & mut [u16]) -> Option < (u16 , usize) > { assert ! (self . len <= other . len ()) ; let mut offset = 0 ; if SIMD_STRIDE_SIZE / 2 <= self . len { let len_minus_stride = self . len - SIMD_STRIDE_SIZE / 2 ; loop { let mut simd = self . simd_at (offset) ; if E :: OPPOSITE_ENDIAN { simd = simd_byte_swap (simd) ; } unsafe { store8_unaligned (other . as_mut_ptr () . add (offset) , simd) ; } if contains_surrogates (simd) { break ; } offset += SIMD_STRIDE_SIZE / 2 ; if offset > len_minus_stride { break ; } } } while offset < self . len { let unit = swap_if_opposite_endian :: < E > (self . at (offset)) ; other [offset] = unit ; if super :: in_range16 (unit , 0xD800 , 0xE000) { return Some ((unit , offset)) ; } offset += 1 ; } None } # [cfg (not (feature = "simd-accel"))] # [inline (always)] fn copy_bmp_to < E : Endian > (& self , other : & mut [u16]) -> Option < (u16 , usize) > { assert ! (self . len <= other . len ()) ; for (i , target) in other . iter_mut () . enumerate () . take (self . len) { let unit = swap_if_opposite_endian :: < E > (self . at (i)) ; * target = unit ; if super :: in_range16 (unit , 0xD800 , 0xE000) { return Some ((unit , i)) ; } } None } }
    };
}

impl_308!()