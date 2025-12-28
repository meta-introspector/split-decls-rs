macro_rules! deps {
    () => {
        Result!();
        Unstructured!();
        Arbitrary!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for Rc < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (Into :: into) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
    };
}

impl_28!()