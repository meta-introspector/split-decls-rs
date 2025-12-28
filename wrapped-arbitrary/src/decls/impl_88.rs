macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        MaxRecursionReached!();
        Result!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a , A > Arbitrary < 'a > for Bound < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { match u . int_in_range :: < u8 > (0 ..= 2) ? { 0 => Ok (Bound :: Included (A :: arbitrary (u) ?)) , 1 => Ok (Bound :: Excluded (A :: arbitrary (u) ?)) , 2 => Ok (Bound :: Unbounded) , _ => unreachable ! () , } } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { Ok (size_hint :: or (size_hint :: and ((1 , Some (1)) , A :: try_size_hint (depth) ?) , (1 , Some (1)) ,)) } }
    };
}

impl_88!()