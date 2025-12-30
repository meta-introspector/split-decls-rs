// Generated macro for shared_new (function)
macro_rules! Depcrateshared_new {
() => {
// Module: crate
// Provides: {"shared_new"}
// Dependencies: {}
fn shared_new < T : WritableAccount > (lamports : u64 , space : usize , owner : & Pubkey) -> T { T :: create (lamports , vec ! [0u8 ; space] , * owner , bool :: default () , Epoch :: default () ,) }
};
}
