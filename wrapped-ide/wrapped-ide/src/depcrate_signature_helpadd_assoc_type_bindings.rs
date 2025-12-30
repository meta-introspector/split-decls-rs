// Generated macro for add_assoc_type_bindings (function)
macro_rules! Depcrate_signature_helpadd_assoc_type_bindings {
() => {
// Module: crate::signature_help
// Provides: {"add_assoc_type_bindings"}
// Dependencies: {}
fn add_assoc_type_bindings (db : & RootDatabase , res : & mut SignatureHelp , tr : Trait , args : ast :: GenericArgList , edition : Edition ,) { if args . syntax () . ancestors () . find_map (ast :: TypeBound :: cast) . is_none () { return ; } let present_bindings = args . generic_args () . filter_map (| arg | match arg { ast :: GenericArg :: AssocTypeArg (arg) => arg . name_ref () . map (| n | n . to_string ()) , _ => None , }) . collect :: < BTreeSet < _ > > () ; let mut buf = String :: new () ; for binding in & present_bindings { buf . clear () ; format_to ! (buf , "{} = …" , binding) ; res . push_generic_param (& buf) ; } for item in tr . items_with_supertraits (db) { if let AssocItem :: TypeAlias (ty) = item { let name = ty . name (db) . display_no_db (edition) . to_smolstr () ; if ! present_bindings . contains (& * name) { buf . clear () ; format_to ! (buf , "{} = …" , name) ; res . push_generic_param (& buf) ; } } } }
};
}
