macro_rules! deps {
    () => {
        Error!();
        ErrorImpl!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Error { fn as_serde_de_error < E : serde :: de :: Error > (& self) -> E { match self . imp . as_ref () { ErrorImpl :: Custom (msg) => E :: custom (msg) , ErrorImpl :: InvalidType { unexpected , expected , } => E :: invalid_type (unexpected . as_serde () , & expected . as_str ()) , ErrorImpl :: InvalidValue { unexpected , expected , } => E :: invalid_value (unexpected . as_serde () , & expected . as_str ()) , ErrorImpl :: InvalidLength { len , expected } => { E :: invalid_length (* len , & expected . as_str ()) } ErrorImpl :: UnknownVariant { variant , expected } => { E :: unknown_variant (variant , expected) } ErrorImpl :: UnknownField { field , expected } => E :: unknown_field (field , expected) , ErrorImpl :: MissingField { field } => E :: missing_field (field) , ErrorImpl :: DuplicateField { field } => E :: duplicate_field (field) , } } }
    };
}

impl_64!();