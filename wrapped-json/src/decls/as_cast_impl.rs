macro_rules! deps {
    () => {
        AsPrimitive!();
        AsCast!();
    };
}

macro_rules! as_cast_impl {
    () => {
        deps!();
        macro_rules ! as_cast_impl { ($ ty : ident , $ method : ident) => { impl AsCast for $ ty { # [inline] fn as_cast < N : AsPrimitive > (n : N) -> Self { n .$ method () } } } ; }
    };
}

as_cast_impl!()