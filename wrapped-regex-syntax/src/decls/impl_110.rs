macro_rules! deps {
    () => {
        Span!();
        Result!();
        CaptureName!();
        Error!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary")] impl arbitrary :: Arbitrary < '_ > for CaptureName { fn arbitrary (u : & mut arbitrary :: Unstructured ,) -> arbitrary :: Result < CaptureName > { let len = u . arbitrary_len :: < char > () ? ; if len == 0 { return Err (arbitrary :: Error :: NotEnoughData) ; } let mut name : String = String :: new () ; for _ in 0 .. len { let ch : char = u . arbitrary () ? ; let cp = u32 :: from (ch) ; let ascii_letter_offset = u8 :: try_from (cp % 26) . unwrap () ; let ascii_letter = b'a' + ascii_letter_offset ; name . push (char :: from (ascii_letter)) ; } Ok (CaptureName { span : u . arbitrary () ? , name , index : u . arbitrary () ? }) } fn size_hint (depth : usize) -> (usize , Option < usize >) { arbitrary :: size_hint :: and_all (& [Span :: size_hint (depth) , usize :: size_hint (depth) , u32 :: size_hint (depth) ,]) } }
    };
}

impl_110!()