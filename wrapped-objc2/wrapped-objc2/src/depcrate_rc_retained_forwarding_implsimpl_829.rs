// Generated macro for impl_829 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_829 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_829"}
// Dependencies: {}
impl < 'a , T : ? Sized > Future for & 'a Retained < T > where & 'a T : Future , { type Output = < & 'a T as Future > :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { < & T > :: poll (Pin :: new (& mut & * * * self) , cx) } }
};
}
