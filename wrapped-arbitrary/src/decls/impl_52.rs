macro_rules! deps {
    () => {
        MaxRecursionReached!();
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Reverse < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > { size_hint :: try_recursion_guard (depth , < A as Arbitrary > :: try_size_hint) } }
    };
}

impl_52!();