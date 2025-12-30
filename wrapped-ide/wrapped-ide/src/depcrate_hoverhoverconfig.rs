// Generated macro for HoverConfig (struct)
macro_rules! Depcrate_hoverHoverConfig {
() => {
// Module: crate::hover
// Provides: {"HoverConfig"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct HoverConfig < 'a > { pub links_in_hover : bool , pub memory_layout : Option < MemoryLayoutHoverConfig > , pub documentation : bool , pub keywords : bool , pub format : HoverDocFormat , pub max_trait_assoc_items_count : Option < usize > , pub max_fields_count : Option < usize > , pub max_enum_variants_count : Option < usize > , pub max_subst_ty_len : SubstTyLen , pub show_drop_glue : bool , pub minicore : MiniCore < 'a > , }
};
}
