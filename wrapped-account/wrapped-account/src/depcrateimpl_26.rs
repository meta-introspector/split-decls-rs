// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl WritableAccount for AccountSharedData { fn set_lamports (& mut self , lamports : u64) { self . lamports = lamports ; } fn data_as_mut_slice (& mut self) -> & mut [u8] { & mut self . data_mut () [..] } fn set_owner (& mut self , owner : Pubkey) { self . owner = owner ; } fn copy_into_owner_from_slice (& mut self , source : & [u8]) { self . owner . as_mut () . copy_from_slice (source) ; } fn set_executable (& mut self , executable : bool) { self . executable = executable ; } fn set_rent_epoch (& mut self , epoch : Epoch) { self . rent_epoch = epoch ; } fn create (lamports : u64 , data : Vec < u8 > , owner : Pubkey , executable : bool , rent_epoch : Epoch ,) -> Self { AccountSharedData { lamports , data : Arc :: new (data) , owner , executable , rent_epoch , } } }
};
}
