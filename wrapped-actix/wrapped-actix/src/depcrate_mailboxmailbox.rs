// Generated macro for Mailbox (struct)
macro_rules! Depcrate_mailboxMailbox {
() => {
// Module: crate::mailbox
// Provides: {"Mailbox"}
// Dependencies: {}
pub struct Mailbox < A > where A : Actor , A :: Context : AsyncContext < A > , { msgs : AddressReceiver < A > , }
};
}
