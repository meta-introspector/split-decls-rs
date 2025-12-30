// Generated macro for retry_interrupted_write_all (function)
macro_rules! Depcrate_write_encoder_testsretry_interrupted_write_all {
() => {
// Module: crate::write::encoder_tests
// Provides: {"retry_interrupted_write_all"}
// Dependencies: {}
# [doc = " Retry writes until all the data is written or an error that isn't Interrupted is returned."] fn retry_interrupted_write_all < W : Write > (w : & mut W , buf : & [u8]) -> io :: Result < () > { let mut bytes_consumed = 0 ; while bytes_consumed < buf . len () { let res = w . write (& buf [bytes_consumed ..]) ; match res { Ok (len) => bytes_consumed += len , Err (e) => match e . kind () { io :: ErrorKind :: Interrupted => continue , _ => return Err (e) , } , } } Ok (()) }
};
}
