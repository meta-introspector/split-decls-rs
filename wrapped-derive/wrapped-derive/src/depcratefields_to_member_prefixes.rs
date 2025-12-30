// Generated macro for fields_to_member_prefixes (function)
macro_rules! Depcratefields_to_member_prefixes {
() => {
// Module: crate
// Provides: {"fields_to_member_prefixes"}
// Dependencies: {}
# [doc = " Gets the [`Prefix`]es for all fields, i.e. the types themselves or paths to prepend to the"] # [doc = " `tls_codec` functions (e.g. a module or type)."] fn fields_to_member_prefixes (fields : & syn :: Fields) -> Result < Vec < Prefix > > { fields . iter () . map (function_prefix) . collect () }
};
}
