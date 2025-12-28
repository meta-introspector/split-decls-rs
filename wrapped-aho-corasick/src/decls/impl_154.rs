macro_rules! deps {
    () => {
        Mask!();
        Vector!();
        Slim!();
        Match!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < V : Vector > Slim < V , 2 > { # [doc = " See Slim<V, 1>::find."] # [inline (always)] pub (crate) unsafe fn find (& self , start : * const u8 , end : * const u8 ,) -> Option < Match > { let len = end . distance (start) ; debug_assert ! (len >= self . minimum_len ()) ; let mut cur = start . add (1) ; let mut prev0 = V :: splat (0xFF) ; while cur <= end . sub (V :: BYTES) { if let Some (m) = self . find_one (cur , end , & mut prev0) { return Some (m) ; } cur = cur . add (V :: BYTES) ; } if cur < end { cur = end . sub (V :: BYTES) ; prev0 = V :: splat (0xFF) ; if let Some (m) = self . find_one (cur , end , & mut prev0) { return Some (m) ; } } None } # [doc = " See Slim<V, 1>::find_one."] # [inline (always)] unsafe fn find_one (& self , cur : * const u8 , end : * const u8 , prev0 : & mut V ,) -> Option < Match > { let c = self . candidate (cur , prev0) ; if ! c . is_zero () { if let Some (m) = self . teddy . verify (cur . sub (1) , end , c) { return Some (m) ; } } None } # [doc = " See Slim<V, 1>::candidate."] # [inline (always)] unsafe fn candidate (& self , cur : * const u8 , prev0 : & mut V) -> V { let chunk = V :: load_unaligned (cur) ; let (res0 , res1) = Mask :: members2 (chunk , self . masks) ; let res0prev0 = res0 . shift_in_one_byte (* prev0) ; let res = res0prev0 . and (res1) ; * prev0 = res0 ; res } }
    };
}

impl_154!()