// Generated macro for impl_356 (impl)
macro_rules! Depcrate_validation_rules_upload_fileimpl_356 {
() => {
// Module: crate::validation::rules::upload_file
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for UploadFile { fn enter_operation_definition (& mut self , ctx : & mut VisitorContext < 'a > , _name : Option < & 'a Name > , operation_definition : & 'a Positioned < OperationDefinition > ,) { for var in & operation_definition . node . variable_definitions { if let Some (ty) = ctx . registry . concrete_type_by_parsed_type (& var . node . var_type . node) { if operation_definition . node . ty != OperationType :: Mutation && ty . name () == "Upload" { ctx . report_error (vec ! [var . pos] , "The Upload type is only allowed to be defined on a mutation" ,) ; } } } } }
};
}
