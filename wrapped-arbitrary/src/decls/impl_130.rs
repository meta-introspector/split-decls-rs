macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
        MaxRecursionReached!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Mutex < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { A :: try_size_hint (depth) } }
    };
}

impl_130!();