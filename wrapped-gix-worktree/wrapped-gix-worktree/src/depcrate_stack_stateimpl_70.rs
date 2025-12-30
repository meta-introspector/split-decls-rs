// Generated macro for impl_70 (impl)
macro_rules! Depcrate_stack_stateimpl_70 {
() => {
// Module: crate::stack::state
// Provides: {"impl_70"}
// Dependencies: {}
# [doc = " Initialization"] impl State { # [doc = " Configure a state to be suitable for checking out files, which only needs access to attribute files read from the index."] # [cfg (feature = "attributes")] pub fn for_checkout (unlink_on_collision : bool , validate : gix_validate :: path :: component :: Options , attributes : Attributes ,) -> Self { State :: CreateDirectoryAndAttributesStack { unlink_on_collision , validate , attributes , } } # [doc = " Configure a state for adding files, with support for ignore files and attribute files."] # [cfg (feature = "attributes")] pub fn for_add (attributes : Attributes , ignore : Ignore) -> Self { State :: AttributesAndIgnoreStack { attributes , ignore } } }
};
}
