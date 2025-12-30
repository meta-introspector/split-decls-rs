// Generated macro for HasAttrs (trait)
macro_rules! Depcrate_ast_traitsHasAttrs {
() => {
// Module: crate::ast_traits
// Provides: {"HasAttrs"}
// Dependencies: {}
# [doc = " A trait for AST nodes having (or not having) attributes."] pub trait HasAttrs { # [doc = " This is `true` if this `HasAttrs` might support 'custom' (proc-macro) inner"] # [doc = " attributes. Attributes like `#![cfg]` and `#![cfg_attr]` are not"] # [doc = " considered 'custom' attributes."] # [doc = ""] # [doc = " If this is `false`, then this `HasAttrs` definitely does"] # [doc = " not support 'custom' inner attributes, which enables some optimizations"] # [doc = " during token collection."] const SUPPORTS_CUSTOM_INNER_ATTRS : bool ; fn attrs (& self) -> & [Attribute] ; fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) ; }
};
}
