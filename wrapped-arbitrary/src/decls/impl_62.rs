macro_rules! deps {
    () => {
        Result!();
        Arbitrary!();
        Unstructured!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for isize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary :: < i64 > () . map (| x | x as isize) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < i64 as Arbitrary > :: size_hint (depth) } }
    };
}

impl_62!();