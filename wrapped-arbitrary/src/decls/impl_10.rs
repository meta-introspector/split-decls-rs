macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        Result!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for Box < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < String as Arbitrary > :: arbitrary (u) . map (| x | x . into_boxed_str ()) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < String as Arbitrary > :: size_hint (depth) } }
    };
}

impl_10!()