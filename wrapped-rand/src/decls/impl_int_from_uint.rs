macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        Rng!();
    };
}

macro_rules! impl_int_from_uint {
    () => {
        deps!();
        macro_rules ! impl_int_from_uint { ($ ty : ty , $ uty : ty) => { impl Distribution <$ ty > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { rng . random ::<$ uty > () as $ ty } } } ; }
    };
}

impl_int_from_uint!()