macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! constant_time_eq_impl {
    () => {
        deps!();
        # [doc = " Generic implementation of `constant_time_eq` and `constant_time_eq_n`."] # [must_use] # [inline (always)] pub (crate) fn constant_time_eq_impl (mut a : & [u8] , mut b : & [u8] , mut tmp : Word) -> bool { if a . len () != b . len () { return false ; } b = & b [.. a . len ()] ; if a . is_empty () { return tmp == 0 ; } # [doc = " Reads and compares a single word from the input, adjusting the slices."] # [doc = " Returns zero if both words are equal, non-zero if any byte is different."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All bit patterns must be valid for type T."] # [must_use] # [inline (always)] unsafe fn cmp_step < T : BitXor < Output = T > > (a : & mut & [u8] , b : & mut & [u8]) -> T { let tmpa = unsafe { read_unaligned_from_slice :: < T > (& a [.. size_of :: < T > ()]) } ; let tmpb = unsafe { read_unaligned_from_slice :: < T > (& b [.. size_of :: < T > ()]) } ; * a = & a [size_of :: < T > () ..] ; * b = & b [size_of :: < T > () ..] ; tmpa ^ tmpb } while a . len () >= size_of :: < Word > () { let cmp = optimizer_hide (unsafe { cmp_step :: < Word > (& mut a , & mut b) }) ; tmp = optimizer_hide (tmp | cmp) ; } while a . len () >= size_of :: < u128 > () { let cmp = optimizer_hide (unsafe { cmp_step :: < u128 > (& mut a , & mut b) } as Word) ; tmp = optimizer_hide (tmp | cmp) ; } if a . len () >= size_of :: < u64 > () { let cmp = optimizer_hide (unsafe { cmp_step :: < u64 > (& mut a , & mut b) } as Word) ; tmp = optimizer_hide (tmp | cmp) ; } if a . len () >= size_of :: < u32 > () { let cmp = optimizer_hide (unsafe { cmp_step :: < u32 > (& mut a , & mut b) } as Word) ; tmp = optimizer_hide (tmp | cmp) ; } if a . len () >= size_of :: < u16 > () { let cmp = optimizer_hide (unsafe { cmp_step :: < u16 > (& mut a , & mut b) } as Word) ; tmp = optimizer_hide (tmp | cmp) ; } if a . len () >= size_of :: < u8 > () { let cmp = optimizer_hide (unsafe { cmp_step :: < u8 > (& mut a , & mut b) } as Word) ; tmp = optimizer_hide (tmp | cmp) ; } tmp == 0 }
    };
}

constant_time_eq_impl!();