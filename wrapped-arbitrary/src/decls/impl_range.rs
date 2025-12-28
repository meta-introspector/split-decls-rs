macro_rules! deps {
    () => {
        Arbitrary!();
        Unstructured!();
        Result!();
        MaxRecursionReached!();
    };
}

macro_rules! impl_range {
    () => {
        deps!();
        macro_rules ! impl_range { ($ range : ty , $ value_closure : expr , $ value_ty : ty , $ fun : ident ($ fun_closure : expr) , $ size_hint_closure : expr) => { impl <'a , A > Arbitrary <'a > for $ range where A : Arbitrary <'a > + Clone + PartialOrd , { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { let value : $ value_ty = Arbitrary :: arbitrary (u) ?; Ok ($ fun (value , $ fun_closure)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { # [allow (clippy :: redundant_closure_call)] $ size_hint_closure (depth) } } } ; }
    };
}

impl_range!()