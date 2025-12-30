// Generated macro for impl_269 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_269 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_269"}
// Dependencies: {}
impl < P , S > Session < P , S > where S : Read , { # [doc = " Creates a new session."] pub fn new (process : P , stream : S) -> io :: Result < Self > { let stream = TryStream :: new (stream) ? ; Ok (Self { proc : process , stream , expect_timeout : Some (Duration :: from_millis (10000)) , expect_lazy : false , }) } pub (crate) fn swap_stream < F , R > (mut self , new : F) -> Result < Session < P , R > , Error > where F : FnOnce (S) -> R , R : Read , { self . stream . flush_in_buffer () ; let buf = self . stream . get_available () . to_owned () ; let stream = self . stream . into_inner () ; let stream = new (stream) ; let mut session = Session :: new (self . proc , stream) ? ; session . stream . keep_in_buffer (& buf) ; Ok (session) } }
};
}
