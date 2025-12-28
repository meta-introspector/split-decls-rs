macro_rules! deps {
    () => {
        Error!();
        Result!();
        AnyValueId!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl std :: fmt :: Debug for AnyValueId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { # [cfg (not (debug_assertions))] { self . type_id . fmt (f) } # [cfg (debug_assertions)] { f . debug_struct (self . type_name) . finish () } } }
    };
}

impl_610!()