// Generated macro for impl_14 (impl)
macro_rules! Depcrate_connectionimpl_14 {
() => {
// Module: crate::connection
// Provides: {"impl_14"}
// Dependencies: {}
impl Future for Connecting { type Output = Result < Connection , ConnectionError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { Pin :: new (& mut self . connected) . poll (cx) . map (| _ | { let conn = self . conn . take () . unwrap () ; let inner = conn . state . lock ("connecting") ; if inner . connected { drop (inner) ; Ok (Connection (conn)) } else { Err (inner . error . clone () . expect ("connected signaled without connection success or error")) } }) } }
};
}
