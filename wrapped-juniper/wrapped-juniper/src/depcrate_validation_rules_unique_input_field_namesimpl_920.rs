// Generated macro for impl_920 (impl)
macro_rules! Depcrate_validation_rules_unique_input_field_namesimpl_920 {
() => {
// Module: crate::validation::rules::unique_input_field_names
// Provides: {"impl_920"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueInputFieldNames < 'a > where S : ScalarValue , { fn enter_object_value (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : SpannedObject < 'a , S >) { self . known_name_stack . push (HashMap :: new ()) ; } fn exit_object_value (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : SpannedObject < 'a , S >) { self . known_name_stack . pop () ; } fn enter_object_field (& mut self , ctx : & mut ValidatorContext < 'a , S > , (field_name , _) : (SpannedInput < 'a , String > , SpannedInput < InputValue < S > >) ,) { if let Some (ref mut known_names) = self . known_name_stack . last_mut () { match known_names . entry (field_name . item) { Entry :: Occupied (e) => { ctx . report_error (& error_message (field_name . item) , & [* e . get () , field_name . span . start] ,) ; } Entry :: Vacant (e) => { e . insert (field_name . span . start) ; } } } } }
};
}
