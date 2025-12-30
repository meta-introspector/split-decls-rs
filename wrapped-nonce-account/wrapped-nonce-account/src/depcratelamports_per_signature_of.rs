// Generated macro for lamports_per_signature_of (function)
macro_rules! Depcratelamports_per_signature_of {
() => {
// Module: crate
// Provides: {"lamports_per_signature_of"}
// Dependencies: {}
pub fn lamports_per_signature_of (account : & AccountSharedData) -> Option < u64 > { match StateMut :: < Versions > :: state (account) . ok () ? . state () { State :: Initialized (data) => Some (data . fee_calculator . lamports_per_signature) , State :: Uninitialized => None , } }
};
}
