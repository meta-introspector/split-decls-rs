// Generated macro for is_valid_for_get_attr (function)
macro_rules! Depcrate_builtin_attrsis_valid_for_get_attr {
() => {
// Module: crate::builtin_attrs
// Provides: {"is_valid_for_get_attr"}
// Dependencies: {}
pub fn is_valid_for_get_attr (name : Symbol) -> bool { BUILTIN_ATTRIBUTE_MAP . get (& name) . is_some_and (| attr | match attr . duplicates { WarnFollowing | ErrorFollowing | ErrorPreceding | FutureWarnFollowing | FutureWarnPreceding => true , DuplicatesOk | WarnFollowingWordOnly => false , }) }
};
}
