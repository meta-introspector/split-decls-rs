// Generated macro for impl_108 (impl)
macro_rules! Depcrate_addressimpl_108 {
() => {
// Module: crate::address
// Provides: {"impl_108"}
// Dependencies: {}
impl fmt :: Display for MailboxError { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MailboxError :: Closed => write ! (fmt , "Mailbox has closed") , MailboxError :: Timeout => write ! (fmt , "Message delivery timed out") , } } }
};
}
