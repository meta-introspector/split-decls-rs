// Generated macro for impl_1230 (impl)
macro_rules! Depcrate_sslimpl_1230 {
() => {
// Module: crate::ssl
// Provides: {"impl_1230"}
// Dependencies: {}
impl < S > SslStream < S > { fn make_error (& mut self , ret : c_int) -> Error { self . check_panic () ; let code = self . ssl . get_error (ret) ; let cause = match code { ErrorCode :: SSL => Some (InnerError :: Ssl (ErrorStack :: get ())) , ErrorCode :: SYSCALL => { let errs = ErrorStack :: get () ; if errs . errors () . is_empty () { self . get_bio_error () . map (InnerError :: Io) } else { Some (InnerError :: Ssl (errs)) } } ErrorCode :: ZERO_RETURN => None , ErrorCode :: WANT_READ | ErrorCode :: WANT_WRITE => { self . get_bio_error () . map (InnerError :: Io) } _ => None , } ; Error { code , cause } } fn check_panic (& mut self) { if let Some (err) = unsafe { bio :: take_panic :: < S > (self . ssl . get_raw_rbio ()) } { resume_unwind (err) } } fn get_bio_error (& mut self) -> Option < io :: Error > { unsafe { bio :: take_error :: < S > (self . ssl . get_raw_rbio ()) } } # [doc = " Returns a shared reference to the underlying stream."] pub fn get_ref (& self) -> & S { unsafe { let bio = self . ssl . get_raw_rbio () ; bio :: get_ref (bio) } } # [doc = " Returns a mutable reference to the underlying stream."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " It is inadvisable to read from or write to the underlying stream as it"] # [doc = " will most likely corrupt the SSL session."] pub fn get_mut (& mut self) -> & mut S { unsafe { let bio = self . ssl . get_raw_rbio () ; bio :: get_mut (bio) } } # [doc = " Returns a shared reference to the `Ssl` object associated with this stream."] pub fn ssl (& self) -> & SslRef { & self . ssl } }
};
}
