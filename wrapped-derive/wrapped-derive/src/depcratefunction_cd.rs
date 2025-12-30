// Generated macro for function_cd (function)
macro_rules! Depcratefunction_cd {
() => {
// Module: crate
// Provides: {"function_cd"}
// Dependencies: {}
# [doc = " Process all attributes of a field and return a single, true or false, `cd_field` value."] # [doc = " This function will return an error in the case of multiple `cd` attributes."] # [cfg (feature = "conditional_deserialization")] fn function_cd (field : & Field) -> Result < bool > { let skip = TlsAttr :: parse_multi (& field . attrs) ? . into_iter () . try_fold (None , | skip , attr | match (skip , attr) { (None , TlsAttr :: CdField) => Ok (Some (true)) , (Some (_) , TlsAttr :: CdField) => Err (syn :: Error :: new (Span :: call_site () , "Attribute `cd_field` specified more than once" ,)) , (skip , _) => Ok (skip) , }) ? . unwrap_or (false) ; Ok (skip) }
};
}
