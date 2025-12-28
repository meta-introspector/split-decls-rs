macro_rules! deps {
    () => {
        Teddy!();
        Vector!();
        Mask!();
        FatMaskBuilder!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl FatMaskBuilder { # [doc = " Update this mask by adding the given byte to the given bucket. The"] # [doc = " given bucket must be in the range 0-15."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `bucket >= 16`."] fn add (& mut self , bucket : usize , byte : u8) { assert ! (bucket < 16) ; let bucket = u8 :: try_from (bucket) . unwrap () ; let byte_lo = usize :: from (byte & 0xF) ; let byte_hi = usize :: from ((byte >> 4) & 0xF) ; if bucket < 8 { self . lo [byte_lo] |= 1 << bucket ; self . hi [byte_hi] |= 1 << bucket ; } else { self . lo [byte_lo + 16] |= 1 << (bucket % 8) ; self . hi [byte_hi + 16] |= 1 << (bucket % 8) ; } } # [doc = " Turn this builder into a vector mask."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `V` represents a vector bigger than what `MaskBytes` can contain."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [inline (always)] unsafe fn build < V : Vector > (& self) -> Mask < V > { assert ! (V :: BYTES <= self . lo . len ()) ; assert ! (V :: BYTES <= self . hi . len ()) ; Mask { lo : V :: load_unaligned (self . lo [..] . as_ptr ()) , hi : V :: load_unaligned (self . hi [..] . as_ptr ()) , } } # [doc = " A convenience function for building `N` vector masks from a fat"] # [doc = " `Teddy` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `V` represents a vector bigger than what `MaskBytes` can contain."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [inline (always)] unsafe fn from_teddy < const BYTES : usize , V : Vector > (teddy : & Teddy < 16 > ,) -> [Mask < V > ; BYTES] { let mut mask_builders = vec ! [FatMaskBuilder :: default () ; BYTES] ; for (bucket_index , bucket) in teddy . buckets . iter () . enumerate () { for pid in bucket . iter () . copied () { let pat = teddy . patterns . get (pid) ; for (i , builder) in mask_builders . iter_mut () . enumerate () { builder . add (bucket_index , pat . bytes () [i]) ; } } } let array = < [FatMaskBuilder ; BYTES] > :: try_from (mask_builders) . unwrap () ; array . map (| builder | builder . build ()) } }
    };
}

impl_173!();