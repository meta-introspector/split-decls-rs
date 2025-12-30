// Generated macro for function_prefix (function)
macro_rules! Depcratefunction_prefix {
() => {
// Module: crate
// Provides: {"function_prefix"}
// Dependencies: {}
# [doc = " Gets the [`Prefix`] for a field, i.e. the type itself or a path to prepend to the `tls_codec`"] # [doc = " functions (e.g. a module or type)."] fn function_prefix (field : & Field) -> Result < Prefix > { let prefix = TlsAttr :: parse_multi (& field . attrs) ? . into_iter () . try_fold (None , | path , attr | match (path , attr) { (None , TlsAttr :: With (p)) => Ok (Some (p)) , (Some (_) , TlsAttr :: With (p)) => Err (syn :: Error :: new_spanned (p , "Attribute `with` specified more than once" ,)) , (path , _) => Ok (path) , }) ? . map (Prefix :: Custom) . unwrap_or_else (| | Prefix :: Type (field . ty . clone ())) ; Ok (prefix) }
};
}
