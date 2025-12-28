macro_rules! deps {
    () => {
        Unstructured!();
        Result!();
        Arbitrary!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for usize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary :: < u64 > () . map (| x | x as usize) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < u64 as Arbitrary > :: size_hint (depth) } }
    };
}

impl_61!()