macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl Variant < '_ > { fn validate (& self) -> Result < () > { check_non_field_attrs (& self . attrs) ? ; if self . attrs . transparent . is_some () { if self . fields . len () != 1 { return Err (Error :: new_spanned (self . original , "#[error(transparent)] requires exactly one field" ,)) ; } if let Some (source) = self . fields . iter () . find_map (| f | f . attrs . source) { return Err (Error :: new_spanned (source . original , "transparent variant can't contain #[source]" ,)) ; } } check_field_attrs (& self . fields) ? ; for field in & self . fields { field . validate () ? ; } Ok (()) } }
    };
}

impl_116!();