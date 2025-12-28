macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for Arc < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (Into :: into) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
    };
}

impl_34!();