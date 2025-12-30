// Generated macro for impl_18 (impl)
macro_rules! Depcrate_stateimpl_18 {
() => {
// Module: crate::state
// Provides: {"impl_18"}
// Dependencies: {}
impl Feature { pub const fn size_of () -> usize { 9 } # [cfg (feature = "bincode")] pub fn from_account_info (account_info : & AccountInfo) -> Result < Self , ProgramError > { if * account_info . owner != id () { return Err (ProgramError :: InvalidAccountOwner) ; } if account_info . data_len () < Feature :: size_of () { return Err (ProgramError :: InvalidAccountData) ; } bincode :: deserialize (& account_info . data . borrow ()) . map_err (| _ | ProgramError :: InvalidAccountData) } }
};
}
