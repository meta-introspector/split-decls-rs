// Generated macro for function_skip (function)
macro_rules! Depcratefunction_skip {
() => {
// Module: crate
// Provides: {"function_skip"}
// Dependencies: {}
# [doc = " Process all attributes of a field and return a single, true or false, `skip` value."] # [doc = " This function will return an error in the case of multiple `skip` attributes."] fn function_skip (field : & Field) -> Result < bool > { let skip = TlsAttr :: parse_multi (& field . attrs) ? . into_iter () . try_fold (None , | skip , attr | match (skip , attr) { (None , TlsAttr :: Skip) => Ok (Some (true)) , (Some (_) , TlsAttr :: Skip) => Err (syn :: Error :: new (Span :: call_site () , "Attribute `skip` specified more than once" ,)) , (skip , _) => Ok (skip) , }) ? . unwrap_or (false) ; Ok (skip) }
};
}
