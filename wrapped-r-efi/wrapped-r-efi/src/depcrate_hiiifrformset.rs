// Generated macro for IfrFormSet (struct)
macro_rules! Depcrate_hiiIfrFormSet {
() => {
// Module: crate::hii
// Provides: {"IfrFormSet"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct IfrFormSet < const N : usize = 0 > { pub header : IfrOpHeader , pub guid : crate :: base :: Guid , pub form_set_title : StringId , pub help : StringId , pub flags : u8 , pub class_guid : [crate :: base :: Guid ; N] , }
};
}
