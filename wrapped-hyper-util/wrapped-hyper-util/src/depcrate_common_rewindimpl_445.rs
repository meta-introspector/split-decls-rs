// Generated macro for impl_445 (impl)
macro_rules! Depcrate_common_rewindimpl_445 {
() => {
// Module: crate::common::rewind
// Provides: {"impl_445"}
// Dependencies: {}
impl < T > Read for Rewind < T > where T : Read + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ > , mut buf : ReadBufCursor < '_ > ,) -> Poll < io :: Result < () > > { if let Some (mut prefix) = self . pre . take () { if ! prefix . is_empty () { let copy_len = cmp :: min (prefix . len () , buf . remaining ()) ; buf . put_slice (& prefix [.. copy_len]) ; prefix . advance (copy_len) ; if ! prefix . is_empty () { self . pre = Some (prefix) ; } return Poll :: Ready (Ok (())) ; } } Pin :: new (& mut self . inner) . poll_read (cx , buf) } }
};
}
