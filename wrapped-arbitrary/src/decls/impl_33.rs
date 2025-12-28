macro_rules! deps {
    () => {
        Result!();
        Unstructured!();
        Arbitrary!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Arc < [A] > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
    };
}

impl_33!()