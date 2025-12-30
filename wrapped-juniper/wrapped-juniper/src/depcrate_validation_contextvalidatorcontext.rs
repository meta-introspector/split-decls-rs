// Generated macro for ValidatorContext (struct)
macro_rules! Depcrate_validation_contextValidatorContext {
() => {
// Module: crate::validation::context
// Provides: {"ValidatorContext"}
// Dependencies: {}
# [doc (hidden)] pub struct ValidatorContext < 'a , S : Debug + 'a > { pub schema : & 'a SchemaType < S > , errors : Vec < RuleError > , type_stack : Vec < Option < & 'a MetaType < S > > > , type_literal_stack : Vec < Option < BorrowedType < 'a > > > , input_type_stack : Vec < Option < & 'a MetaType < S > > > , input_type_literal_stack : Vec < Option < BorrowedType < 'a > > > , parent_type_stack : Vec < Option < & 'a MetaType < S > > > , fragment_names : HashSet < & 'a str > , }
};
}
