// Generated macro for shared_new_ref (function)
macro_rules! Depcrateshared_new_ref {
() => {
// Module: crate
// Provides: {"shared_new_ref"}
// Dependencies: {}
fn shared_new_ref < T : WritableAccount > (lamports : u64 , space : usize , owner : & Pubkey ,) -> Rc < RefCell < T > > { Rc :: new (RefCell :: new (shared_new :: < T > (lamports , space , owner))) }
};
}
