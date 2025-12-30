// Generated macro for create_with_data (function)
macro_rules! Depcrate_propertylistcreate_with_data {
() => {
// Module: crate::propertylist
// Provides: {"create_with_data"}
// Dependencies: {}
pub fn create_with_data (data : CFData , options : CFPropertyListMutabilityOptions ,) -> Result < (* const c_void , CFPropertyListFormat) , CFError > { unsafe { let mut error : CFErrorRef = ptr :: null_mut () ; let mut format : CFPropertyListFormat = 0 ; let property_list = CFPropertyListCreateWithData (kCFAllocatorDefault , data . as_concrete_TypeRef () , options , & mut format , & mut error ,) ; if property_list . is_null () { Err (TCFType :: wrap_under_create_rule (error)) } else { Ok ((property_list , format)) } } }
};
}
