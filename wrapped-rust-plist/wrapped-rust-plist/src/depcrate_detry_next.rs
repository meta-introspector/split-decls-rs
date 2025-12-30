// Generated macro for try_next (macro)
macro_rules! Depcrate_detry_next {
() => {
// Module: crate::de
// Provides: {"try_next"}
// Dependencies: {}
macro_rules ! try_next { ($ next : expr) => { match $ next { Some (Ok (event)) => event , Some (Err (err)) => return Err (err) ?, None => return Err (ErrorKind :: UnexpectedEndOfEventStream . without_position ()) ?, } } ; }
};
}
