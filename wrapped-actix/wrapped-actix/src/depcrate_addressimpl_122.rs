// Generated macro for impl_122 (impl)
macro_rules! Depcrate_addressimpl_122 {
() => {
// Module: crate::address
// Provides: {"impl_122"}
// Dependencies: {}
impl < A : Actor > WeakAddr < A > { # [doc = " Attempts to upgrade the [`WeakAddr<A>`] pointer to an [`Addr<A>`]."] # [doc = ""] # [doc = " Returns `None` if the actor has since been dropped or the"] # [doc = " underlying address is disconnected."] pub fn upgrade (& self) -> Option < Addr < A > > { match self . wtx . upgrade () { Some (tx) => { if tx . connected () { Some (Addr :: new (tx)) } else { None } } None => None , } } pub fn recipient < M > (self) -> WeakRecipient < M > where A : Handler < M > , A :: Context : ToEnvelope < A , M > , M : Message + Send + 'static , M :: Result : Send , { self . into () } }
};
}
