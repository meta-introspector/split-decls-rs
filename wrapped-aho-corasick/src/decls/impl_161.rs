macro_rules! deps {
    () => {
        FatVector!();
        Match!();
        Fat!();
        Mask!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < V : FatVector > Fat < V , 3 > { # [doc = " See `Fat<V, 1>::find`."] # [inline (always)] pub (crate) unsafe fn find (& self , start : * const u8 , end : * const u8 ,) -> Option < Match > { let len = end . distance (start) ; debug_assert ! (len >= self . minimum_len ()) ; let mut cur = start . add (2) ; let mut prev0 = V :: splat (0xFF) ; let mut prev1 = V :: splat (0xFF) ; while cur <= end . sub (V :: Half :: BYTES) { if let Some (m) = self . find_one (cur , end , & mut prev0 , & mut prev1) { return Some (m) ; } cur = cur . add (V :: Half :: BYTES) ; } if cur < end { cur = end . sub (V :: Half :: BYTES) ; prev0 = V :: splat (0xFF) ; prev1 = V :: splat (0xFF) ; if let Some (m) = self . find_one (cur , end , & mut prev0 , & mut prev1) { return Some (m) ; } } None } # [doc = " See `Fat<V, 1>::find_one`."] # [inline (always)] unsafe fn find_one (& self , cur : * const u8 , end : * const u8 , prev0 : & mut V , prev1 : & mut V ,) -> Option < Match > { let c = self . candidate (cur , prev0 , prev1) ; if ! c . is_zero () { if let Some (m) = self . teddy . verify (cur . sub (2) , end , c) { return Some (m) ; } } None } # [doc = " See `Fat<V, 1>::candidate`."] # [inline (always)] unsafe fn candidate (& self , cur : * const u8 , prev0 : & mut V , prev1 : & mut V ,) -> V { let chunk = V :: load_half_unaligned (cur) ; let (res0 , res1 , res2) = Mask :: members3 (chunk , self . masks) ; let res0prev0 = res0 . half_shift_in_two_bytes (* prev0) ; let res1prev1 = res1 . half_shift_in_one_byte (* prev1) ; let res = res0prev0 . and (res1prev1) . and (res2) ; * prev0 = res0 ; * prev1 = res1 ; res } }
    };
}

impl_161!()