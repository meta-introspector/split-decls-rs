// Generated macro for IfrVarstore (struct)
macro_rules! Depcrate_hiiIfrVarstore {
() => {
// Module: crate::hii
// Provides: {"IfrVarstore"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct IfrVarstore < const N : usize = 0 > { pub header : IfrOpHeader , pub guid : crate :: base :: Guid , pub var_store_id : VarstoreId , pub size : u16 , pub name : [u8 ; N] , }
};
}
