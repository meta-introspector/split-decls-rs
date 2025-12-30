// Generated macro for IoapicState (struct)
macro_rules! Depcrate_x86_64IoapicState {
() => {
// Module: crate::x86_64
// Provides: {"IoapicState"}
// Dependencies: {}
# [repr (C)] # [derive (Copy)] pub struct IoapicState { pub base_address : u64 , pub ioregsel : u32 , pub id : u32 , pub irr : u32 , pub pad : u32 , pub redirtbl : [Union_Unnamed3 ; 24usize] , }
};
}
