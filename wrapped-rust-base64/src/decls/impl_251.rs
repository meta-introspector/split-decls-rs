macro_rules! deps {
    () => {
        DecodePaddingMode!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl distributions :: Distribution < DecodePaddingMode > for distributions :: Standard { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> DecodePaddingMode { match rng . gen_range (0 ..= 2) { 0 => DecodePaddingMode :: Indifferent , 1 => DecodePaddingMode :: RequireCanonical , _ => DecodePaddingMode :: RequireNone , } } }
    };
}

impl_251!()