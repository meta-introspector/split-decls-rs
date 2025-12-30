// Generated macro for impl_105 (impl)
macro_rules! Depcrate_common_renameimpl_105 {
() => {
// Module: crate::common::rename
// Provides: {"impl_105"}
// Dependencies: {}
impl Policy { # [doc = " Applies this [`Policy`] to the given `name`."] pub (crate) fn apply (& self , name : & str) -> String { match self { Self :: None => name . into () , Self :: CamelCase => to_camel_case (name) , Self :: SnakeCase => to_snake_case (name , false) , Self :: ScreamingSnakeCase => to_snake_case (name , true) , } } }
};
}
