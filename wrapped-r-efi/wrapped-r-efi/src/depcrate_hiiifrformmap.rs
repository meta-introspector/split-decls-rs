// Generated macro for IfrFormMap (struct)
macro_rules! Depcrate_hiiIfrFormMap {
() => {
// Module: crate::hii
// Provides: {"IfrFormMap"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct IfrFormMap < const N : usize = 0 > { pub header : IfrOpHeader , pub form_id : FormId , pub methods : [IfrFormMapMethod ; N] , }
};
}
