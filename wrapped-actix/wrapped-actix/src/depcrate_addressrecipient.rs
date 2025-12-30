// Generated macro for Recipient (struct)
macro_rules! Depcrate_addressRecipient {
() => {
// Module: crate::address
// Provides: {"Recipient"}
// Dependencies: {}
# [doc = " The [`Recipient`] type allows to send one specific message to an actor."] # [doc = ""] # [doc = " You can get a recipient using the `Addr::recipient()` method. It is possible"] # [doc = " to use the `Clone::clone()` method to get a cloned recipient."] pub struct Recipient < M : Message > where M : Message + Send , M :: Result : Send , { tx : Box < dyn Sender < M > + Sync > , }
};
}
