macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Field < '_ > { fn validate (& self) -> Result < () > { if let Some (unexpected_display_attr) = if let Some (display) = & self . attrs . display { Some (display . original) } else if let Some (fmt) = & self . attrs . fmt { Some (fmt . original) } else { None } { return Err (Error :: new_spanned (unexpected_display_attr , "not expected here; the #[error(...)] attribute belongs on top of a struct or an enum variant" ,)) ; } Ok (()) } }
    };
}

impl_117!();