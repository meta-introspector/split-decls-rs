// Generated macro for impl_479 (impl)
macro_rules! Depcrate_ioimpl_479 {
() => {
// Module: crate::io
// Provides: {"impl_479"}
// Dependencies: {}
impl < T , E , A > ActorFuture < A > for WriterDrain < T , E > where T : AsyncWrite + Unpin , E : From < io :: Error > , A : Actor , A :: Context : AsyncContext < A > , { type Output = () ; fn poll (self : Pin < & mut Self > , _ : & mut A , _ : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let this = self . get_mut () ; let mut inner = this . inner . 0 . borrow_mut () ; if inner . error . is_some () { return Poll :: Ready (()) ; } let mut io = this . inner . 1 . borrow_mut () ; while ! inner . buffer . is_empty () { match Pin :: new (io . deref_mut ()) . poll_write (task , & inner . buffer) { Poll :: Ready (Ok (n)) => { if n == 0 { inner . error = Some (io :: Error :: new (io :: ErrorKind :: WriteZero , "failed to write frame to transport" ,) . into () ,) ; return Poll :: Ready (()) ; } let _ = inner . buffer . split_to (n) ; } Poll :: Ready (Err (ref e)) if e . kind () == io :: ErrorKind :: WouldBlock => { return if inner . buffer . len () < inner . low { Poll :: Ready (()) } else { Poll :: Pending } ; } Poll :: Ready (Err (e)) => { inner . error = Some (e . into ()) ; return Poll :: Ready (()) ; } Poll :: Pending => return Poll :: Pending , } } Poll :: Ready (()) } }
};
}
