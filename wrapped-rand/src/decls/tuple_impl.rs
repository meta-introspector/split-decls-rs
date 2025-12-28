macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        StandardUniform!();
    };
}

macro_rules! tuple_impl {
    () => {
        deps!();
        # [doc = " Implement `Distribution<(A, B, C, ...)> for StandardUniform`, using the list of"] # [doc = " identifiers"] macro_rules ! tuple_impl { ($ ($ tyvar : ident) *) => { impl < $ ($ tyvar ,) * > Distribution < ($ ($ tyvar ,) *) > for StandardUniform where $ (StandardUniform : Distribution < $ tyvar >,) * { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> ($ ($ tyvar ,) *) { let out = ($ (rng . random ::<$ tyvar > () ,) *) ; let _rng = rng ; out } } } }
    };
}

tuple_impl!();