// Generated macro for extract_clippy_lint (function)
macro_rules! Depcrate_attrs_utilsextract_clippy_lint {
() => {
// Module: crate::attrs::utils
// Provides: {"extract_clippy_lint"}
// Dependencies: {}
# [doc = " Returns the lint name if it is clippy lint."] pub (super) fn extract_clippy_lint (lint : & MetaItemInner) -> Option < Symbol > { match namespace_and_lint (lint) { (Some (sym :: clippy) , name) => name , _ => None , } }
};
}
