macro_rules! deps {
    () => {
        Struct!();
        Display!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Struct < '_ > { fn validate (& self) -> Result < () > { check_non_field_attrs (& self . attrs) ? ; if let Some (transparent) = self . attrs . transparent { if self . fields . len () != 1 { return Err (Error :: new_spanned (transparent . original , "#[error(transparent)] requires exactly one field" ,)) ; } if let Some (source) = self . fields . iter () . find_map (| f | f . attrs . source) { return Err (Error :: new_spanned (source . original , "transparent error struct can't contain #[source]" ,)) ; } } if let Some (fmt) = & self . attrs . fmt { return Err (Error :: new_spanned (fmt . original , "#[error(fmt = ...)] is only supported in enums; for a struct, handwrite your own Display impl" ,)) ; } check_field_attrs (& self . fields) ? ; for field in & self . fields { field . validate () ? ; } Ok (()) } }
    };
}

impl_114!()