macro_rules! deps {
    () => {
        Result!();
        Arbitrary!();
        Unstructured!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for OsString { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < String as Arbitrary > :: arbitrary (u) . map (From :: from) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < String as Arbitrary > :: size_hint (depth) } }
    };
}

impl_118!();