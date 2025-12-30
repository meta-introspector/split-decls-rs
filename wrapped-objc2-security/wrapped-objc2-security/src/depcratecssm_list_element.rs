// Generated macro for cssm_list_element (struct)
macro_rules! Depcratecssm_list_element {
() => {
// Module: crate
// Provides: {"cssm_list_element"}
// Dependencies: {}
# [allow (non_camel_case_types)] # [allow (non_snake_case)] # [cfg (all (feature = "cssmtype" , feature = "SecAsn1Types" , feature = "cssmconfig"))] # [derive (Copy , Clone)] # [repr (C)] pub struct cssm_list_element { pub NextElement : * mut cssm_list_element , pub WordID : CSSM_WORDID_TYPE , pub ElementType : CSSM_LIST_ELEMENT_TYPE , pub Element : cssm_list_element_Element , }
};
}
