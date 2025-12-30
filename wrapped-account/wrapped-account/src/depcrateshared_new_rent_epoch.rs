// Generated macro for shared_new_rent_epoch (function)
macro_rules! Depcrateshared_new_rent_epoch {
() => {
// Module: crate
// Provides: {"shared_new_rent_epoch"}
// Dependencies: {}
fn shared_new_rent_epoch < T : WritableAccount > (lamports : u64 , space : usize , owner : & Pubkey , rent_epoch : Epoch ,) -> T { T :: create (lamports , vec ! [0u8 ; space] , * owner , bool :: default () , rent_epoch ,) }
};
}
