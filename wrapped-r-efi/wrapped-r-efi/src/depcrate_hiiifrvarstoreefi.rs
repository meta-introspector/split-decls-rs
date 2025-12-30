// Generated macro for IfrVarstoreEfi (struct)
macro_rules! Depcrate_hiiIfrVarstoreEfi {
() => {
// Module: crate::hii
// Provides: {"IfrVarstoreEfi"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct IfrVarstoreEfi < const N : usize = 0 > { pub header : IfrOpHeader , pub var_store_id : VarstoreId , pub guid : crate :: base :: Guid , pub attributes : u32 , pub size : u16 , pub name : [u8 ; N] , }
};
}
