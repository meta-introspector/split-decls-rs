// Generated macro for impl_128 (impl)
macro_rules! Depcrate_addressimpl_128 {
() => {
// Module: crate::address
// Provides: {"impl_128"}
// Dependencies: {}
impl < M > Recipient < M > where M : Message + Send , M :: Result : Send , { # [doc = " Creates a new recipient."] pub (crate) fn new (tx : Box < dyn Sender < M > + Sync >) -> Recipient < M > { Recipient { tx } } # [doc = " Sends a message."] # [doc = ""] # [doc = " The message is always queued, even if the mailbox for the receiver is full. If the mailbox"] # [doc = " is closed, the message is silently dropped."] pub fn do_send (& self , msg : M) { let _ = self . tx . do_send (msg) ; } # [doc = " Attempts to send a message."] # [doc = ""] # [doc = " This method fails if the actor's mailbox is full or closed. This method registers the"] # [doc = " current task in the receivers queue."] pub fn try_send (& self , msg : M) -> Result < () , SendError < M > > { self . tx . try_send (msg) } # [doc = " Sends a message and asynchronously wait for a response."] # [doc = ""] # [doc = " The communication channel to the actor is bounded. If the returned `RecipientRequest` object"] # [doc = " gets dropped, the message is cancelled."] pub fn send (& self , msg : M) -> RecipientRequest < M > { match self . tx . send (msg) { Ok (rx) => RecipientRequest :: new (Some (rx) , None) , Err (SendError :: Full (msg)) => RecipientRequest :: new (None , Some ((self . tx . boxed () , msg))) , Err (SendError :: Closed (_)) => RecipientRequest :: new (None , None) , } } pub fn connected (& self) -> bool { self . tx . connected () } # [doc = " Returns a downgraded `WeakRecipient`"] pub fn downgrade (& self) -> WeakRecipient < M > { WeakRecipient { wtx : self . tx . downgrade () , } } }
};
}
