// Generated macro for impl_36 (impl)
macro_rules! Depcrate_yielderimpl_36 {
() => {
// Module: crate::yielder
// Provides: {"impl_36"}
// Dependencies: {}
impl < T > Future for Send < T > { type Output = () ; fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < () > { if self . value . is_none () { return Poll :: Ready (()) ; } STORE . with (| cell | { let ptr = cell . get () as * mut Option < T > ; let option_ref = unsafe { ptr . as_mut () } . expect ("invalid usage") ; if option_ref . is_none () { * option_ref = self . value . take () ; } Poll :: Pending }) } }
};
}
