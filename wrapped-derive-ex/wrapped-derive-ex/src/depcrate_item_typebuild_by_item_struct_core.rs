// Generated macro for build_by_item_struct_core (function)
macro_rules! Depcrate_item_typebuild_by_item_struct_core {
() => {
// Module: crate::item_type
// Provides: {"build_by_item_struct_core"}
// Dependencies: {}
fn build_by_item_struct_core (attr : Option < TokenStream > , item : & ItemStruct , kinds : & mut HelperAttributeKinds ,) -> Result < TokenStream > { let es = DeriveEntry :: from_root (attr , & item . attrs) ? ; kinds . extend (& es) ; let hattrs = HelperAttributes :: from_attrs (& item . attrs , AttributeTarget :: Type , & kinds . without_derive_ex () ,) ? ; let fields = FieldEntry :: from_fields (& item . fields , kinds) ? ; let mut ts_all = TokenStream :: new () ; for e in es { let result = match e . kind { DeriveItemKind :: BinaryOp (op) => build_binary_op (item , op , & e , & fields) , DeriveItemKind :: AssignOp (op) => build_assign_op (item , op , & e , & fields) , DeriveItemKind :: UnaryOp (op) => build_unary_op (item , op , & e , & fields) , DeriveItemKind :: CompareOp (op) => { build_compare_op_for_struct (op , item , & e , & hattrs , & fields) } DeriveItemKind :: Copy => build_copy_for_struct (item , & e , & fields) , DeriveItemKind :: Clone => build_clone_for_struct (item , & e , & fields) , DeriveItemKind :: Debug => build_debug_for_struct (item , & e , & hattrs , & fields) , DeriveItemKind :: Default => build_default_for_struct (item , & e , & hattrs , & fields) , DeriveItemKind :: Deref | DeriveItemKind :: DerefMut => { build_deref_for_struct (item , & e , & fields) } } ; ts_all . extend (e . apply_dump (result)) ; } Ok (ts_all) }
};
}
