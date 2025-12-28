macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! TypeParameter {
    () => {
        deps!();
        struct TypeParameter { bound_generic_params : ThinVec < ast :: GenericParam > , ty : Box < ast :: Ty > , }
    };
}

TypeParameter!();