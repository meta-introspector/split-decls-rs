macro_rules! deps {
    () => {
        Unexpected!();
        Error!();
        ErrorImpl!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl serde :: de :: Error for Error { fn custom < T : Display > (msg : T) -> Self { let imp = Box :: new (ErrorImpl :: Custom (msg . to_string ())) ; Error { imp } } fn invalid_type (unexpected : serde :: de :: Unexpected , expected : & dyn Expected) -> Self { let imp = Box :: new (ErrorImpl :: InvalidType { unexpected : Unexpected :: from_serde (unexpected) , expected : expected . to_string () , }) ; Error { imp } } fn invalid_value (unexpected : serde :: de :: Unexpected , expected : & dyn Expected) -> Self { let imp = Box :: new (ErrorImpl :: InvalidValue { unexpected : Unexpected :: from_serde (unexpected) , expected : expected . to_string () , }) ; Error { imp } } fn invalid_length (len : usize , expected : & dyn Expected) -> Self { let imp = Box :: new (ErrorImpl :: InvalidLength { len , expected : expected . to_string () , }) ; Error { imp } } fn unknown_variant (variant : & str , expected : & 'static [& 'static str]) -> Self { let imp = Box :: new (ErrorImpl :: UnknownVariant { variant : variant . to_owned () , expected , }) ; Error { imp } } fn unknown_field (field : & str , expected : & 'static [& 'static str]) -> Self { let imp = Box :: new (ErrorImpl :: UnknownField { field : field . to_owned () , expected , }) ; Error { imp } } fn missing_field (field : & 'static str) -> Self { let imp = Box :: new (ErrorImpl :: MissingField { field }) ; Error { imp } } fn duplicate_field (field : & 'static str) -> Self { let imp = Box :: new (ErrorImpl :: DuplicateField { field }) ; Error { imp } } }
    };
}

impl_63!();