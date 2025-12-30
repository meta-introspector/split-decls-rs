// Generated macro for build_by_item_struct (function)
macro_rules! Depcrate_item_typebuild_by_item_struct {
() => {
// Module: crate::item_type
// Provides: {"build_by_item_struct"}
// Dependencies: {}
pub fn build_by_item_struct (attr : TokenStream , item : & mut ItemStruct) -> Result < TokenStream > { let mut kinds = HelperAttributeKinds :: new (true) ; let result = build_by_item_struct_core (Some (attr) , item , & mut kinds) ; remove_attrs (& mut item . attrs , & kinds) ; for field in & mut item . fields { remove_attrs (& mut field . attrs , & kinds) } result }
};
}
