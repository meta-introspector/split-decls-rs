// Generated macro for emit_fatal_malformed_builtin_attribute (function)
macro_rules! Depcrate_validate_attremit_fatal_malformed_builtin_attribute {
() => {
// Module: crate::validate_attr
// Provides: {"emit_fatal_malformed_builtin_attribute"}
// Dependencies: {}
pub fn emit_fatal_malformed_builtin_attribute (psess : & ParseSess , attr : & Attribute , name : Symbol ,) -> ! { let template = BUILTIN_ATTRIBUTE_MAP . get (& name) . expect ("builtin attr defined") . template ; emit_malformed_attribute (psess , attr . style , attr . span , name , template) ; FatalError . raise () }
};
}
