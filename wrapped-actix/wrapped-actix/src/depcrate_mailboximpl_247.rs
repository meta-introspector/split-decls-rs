// Generated macro for impl_247 (impl)
macro_rules! Depcrate_mailboximpl_247 {
() => {
// Module: crate::mailbox
// Provides: {"impl_247"}
// Dependencies: {}
impl < A > fmt :: Debug for Mailbox < A > where A : Actor , A :: Context : AsyncContext < A > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Mailbox") . field ("capacity" , & self . capacity ()) . finish () } }
};
}
