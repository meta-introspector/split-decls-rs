// Generated macro for map_container_to_text (function)
macro_rules! Depcrate_doc_lazy_continuationmap_container_to_text {
() => {
// Module: crate::doc::lazy_continuation
// Provides: {"map_container_to_text"}
// Dependencies: {}
fn map_container_to_text (c : & super :: Container) -> & 'static str { match c { super :: Container :: Blockquote => "> " , super :: Container :: List (indent) => & "                  " [0 .. * indent] , } }
};
}
