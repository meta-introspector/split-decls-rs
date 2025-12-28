macro_rules! deps {
    () => {
        Uts46Mapper!();
        Uts46MapperBorrowed!();
        ComposingNormalizerBorrowed!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Uts46MapperBorrowed < 'static > { # [doc = " Cheaply converts a [`Uts46MapperBorrowed<'static>`] into a [`Uts46Mapper`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`Uts46Mapper`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`Uts46MapperBorrowed`]."] pub const fn static_to_owned (self) -> Uts46Mapper { Uts46Mapper { normalizer : self . normalizer . static_to_owned () , } } # [doc = " Construct with compiled data."] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Uts46MapperBorrowed { normalizer : ComposingNormalizerBorrowed :: new_uts46 () , } } }
    };
}

impl_58!();