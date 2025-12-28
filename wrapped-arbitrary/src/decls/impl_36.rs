macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Vec < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
    };
}

impl_36!();