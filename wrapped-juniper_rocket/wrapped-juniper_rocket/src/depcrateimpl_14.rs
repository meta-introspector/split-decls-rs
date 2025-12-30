// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < S : ScalarValue > GraphQLContext < '_ , S > { fn query (& mut self , value : String) { if self . query . is_some () { let error = Error :: from (ErrorKind :: Duplicate) . with_name ("query") ; self . errors . push (error) } else { self . query = Some (value) ; } } fn operation_name (& mut self , value : String) { if self . operation_name . is_some () { let error = Error :: from (ErrorKind :: Duplicate) . with_name ("operationName") ; self . errors . push (error) } else { self . operation_name = Some (value) ; } } fn variables (& mut self , value : String) { if self . variables . is_some () { let error = Error :: from (ErrorKind :: Duplicate) . with_name ("variables") ; self . errors . push (error) } else { let parse_result = serde_json :: from_str :: < InputValue < S > > (& value) ; match parse_result { Ok (variables) => self . variables = Some (variables) , Err (e) => { let error = Error :: from (ErrorKind :: Validation (Cow :: Owned (e . to_string ()))) . with_name ("variables") ; self . errors . push (error) ; } } } } }
};
}
