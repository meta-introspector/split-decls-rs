// Generated macro for create_data (function)
macro_rules! Depcrate_propertylistcreate_data {
() => {
// Module: crate::propertylist
// Provides: {"create_data"}
// Dependencies: {}
pub fn create_data (property_list : * const c_void , format : CFPropertyListFormat ,) -> Result < CFData , CFError > { unsafe { let mut error : CFErrorRef = ptr :: null_mut () ; let data_ref = CFPropertyListCreateData (kCFAllocatorDefault , property_list , format , 0 , & mut error) ; if data_ref . is_null () { Err (TCFType :: wrap_under_create_rule (error)) } else { Ok (TCFType :: wrap_under_create_rule (data_ref)) } } }
};
}
