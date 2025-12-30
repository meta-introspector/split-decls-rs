// Generated macro for stream_to_io_write (function)
macro_rules! Depcrate_to_iostream_to_io_write {
() => {
// Module: crate::to_io
// Provides: {"stream_to_io_write"}
// Dependencies: {}
# [doc = "\nStream a value as JSON to an underlying writer.\n"] pub fn stream_to_io_write (io : impl Write , v : impl sval :: Value) -> Result < () , Error > { struct IoToFmt < W > { io : W , err : Option < io :: Error > , } impl < W : Write > fmt :: Write for IoToFmt < W > { fn write_str (& mut self , v : & str) -> fmt :: Result { let mut buf = v . as_bytes () ; while buf . len () > 0 { match self . io . write (buf) { Ok (0) => break , Ok (n) => { buf = & buf [n ..] ; } Err (e) if e . kind () == io :: ErrorKind :: Interrupted => continue , Err (e) => { self . err = Some (e) ; return Err (fmt :: Error) ; } } } Ok (()) } } let mut io = IoToFmt { io , err : None } ; match crate :: stream_to_fmt_write (& mut io , v) { Ok (()) => Ok (()) , Err (mut e) => { if let Some (io) = io . err { e . kind = ErrorKind :: IO (io) ; } Err (e) } } }
};
}
