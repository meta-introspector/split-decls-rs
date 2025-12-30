// Generated macro for impl_191 (impl)
macro_rules! Depcrate_nonblockimpl_191 {
() => {
// Module: crate::nonblock
// Provides: {"impl_191"}
// Dependencies: {}
impl Future for MROuter { type Output = Result < Message , Error > ; fn poll (self : pin :: Pin < & mut Self > , ctx : & mut task :: Context) -> task :: Poll < Self :: Output > { let mut inner = self . 0 . lock () . unwrap () ; let r = mem :: replace (& mut * inner , MRInner :: Neither) ; if let MRInner :: Ready (r) = r { task :: Poll :: Ready (r) } else { * inner = MRInner :: Pending (ctx . waker () . clone ()) ; return task :: Poll :: Pending } } }
};
}
