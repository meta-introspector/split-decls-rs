macro_rules! deps {
    () => {
        Result!();
        Arbitrary!();
        Unstructured!();
        MaxRecursionReached!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Cell < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { < A as Arbitrary < 'a > > :: try_size_hint (depth) } }
    };
}

impl_46!()