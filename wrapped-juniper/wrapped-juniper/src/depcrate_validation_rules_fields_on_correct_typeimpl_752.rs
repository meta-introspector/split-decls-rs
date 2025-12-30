// Generated macro for impl_752 (impl)
macro_rules! Depcrate_validation_rules_fields_on_correct_typeimpl_752 {
() => {
// Module: crate::validation::rules::fields_on_correct_type
// Provides: {"impl_752"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for FieldsOnCorrectType where S : ScalarValue , { fn enter_operation_definition (& mut self , context : & mut ValidatorContext < 'a , S > , operation : & 'a Spanning < Operation < S > > ,) { if let OperationType :: Subscription = operation . item . operation_type { for selection in & operation . item . selection_set { if let Selection :: Field (field) = selection { if field . item . name . item == "__typename" { context . report_error ("`__typename` may not be included as a root \
                             field in a subscription operation" , & [field . item . name . span . start] ,) ; } } } } } fn enter_field (& mut self , context : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > > ,) { { if let Some (parent_type) = context . parent_type () { let field_name = & field . item . name ; let type_name = parent_type . name () . map (ArcStr :: as_str) . unwrap_or ("<unknown>") ; if parent_type . field_by_name (field_name . item) . is_none () { if let MetaType :: Union (..) = * parent_type { if field_name . item == "__typename" { return ; } } context . report_error (& error_message (field_name . item , type_name) , & [field_name . span . start] ,) ; } } } } }
};
}
