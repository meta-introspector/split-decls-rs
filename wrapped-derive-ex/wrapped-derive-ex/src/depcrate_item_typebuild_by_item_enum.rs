// Generated macro for build_by_item_enum (function)
macro_rules! Depcrate_item_typebuild_by_item_enum {
() => {
// Module: crate::item_type
// Provides: {"build_by_item_enum"}
// Dependencies: {}
pub fn build_by_item_enum (attr : TokenStream , item : & mut ItemEnum) -> Result < TokenStream > { let mut kinds = HelperAttributeKinds :: new (true) ; let result = build_by_item_enum_core (Some (attr) , item , & mut kinds) ; remove_attrs (& mut item . attrs , & kinds) ; for variant in & mut item . variants { remove_attrs (& mut variant . attrs , & kinds) ; for field in & mut variant . fields { remove_attrs (& mut field . attrs , & kinds) } } result }
};
}
