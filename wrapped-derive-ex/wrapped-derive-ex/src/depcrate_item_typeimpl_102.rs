// Generated macro for impl_102 (impl)
macro_rules! Depcrate_item_typeimpl_102 {
() => {
// Module: crate::item_type
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > FieldEntry < 'a > { fn new (index : usize , field : & 'a Field , kinds : & HelperAttributeKinds) -> Result < Self > { Ok (Self { index , field , hattrs : HelperAttributes :: from_attrs (& field . attrs , AttributeTarget :: Field , kinds) ? , }) } fn from_fields (fields : & 'a Fields , kinds : & HelperAttributeKinds) -> Result < Vec < Self > > { fields . iter () . enumerate () . map (| (index , field) | Self :: new (index , field , kinds)) . collect () } fn span (& self) -> Span { self . field . ty . span () } fn member (& self) -> TokenStream { if let Some (ident) = & self . field . ident { let mut ident = ident . clone () ; ident . set_span (self . span ()) ; quote ! (# ident) } else { let mut index = Index :: from (self . index) ; index . span = self . span () ; quote ! (# index) } } fn make_ident (& self , prefix : & str) -> Ident { if let Some (ident) = & self . field . ident { format_ident ! ("{}_{}" , prefix , ident) } else { format_ident ! ("{}_{}" , prefix , self . index) } } fn push_bounds_to (& self , use_bounds : bool , kind : DeriveItemKind , wcb : & mut WhereClauseBuilder) { if self . hattrs . push_bounds_to (use_bounds , kind , wcb) { wcb . push_bounds_for_field (self . field) } } }
};
}
