// Generated macro for try_with_interrupt (macro)
macro_rules! Depcrate_io_allow_stdtry_with_interrupt {
() => {
// Module: crate::io::allow_std
// Provides: {"try_with_interrupt"}
// Dependencies: {}
macro_rules ! try_with_interrupt { ($ e : expr) => { loop { match $ e { Ok (e) => { break e ; } Err (ref e) if e . kind () == :: std :: io :: ErrorKind :: Interrupted => { continue ; } Err (e) => { return Poll :: Ready (Err (e)) ; } } } } ; }
};
}
