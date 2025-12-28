macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        Rng!();
    };
}

macro_rules! impl_nzint {
    () => {
        deps!();
        macro_rules ! impl_nzint { ($ ty : ty , $ new : path) => { impl Distribution <$ ty > for StandardUniform { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { loop { if let Some (nz) = $ new (rng . random ()) { break nz ; } } } } } ; }
    };
}

impl_nzint!()