// Generated macro for impl_63 (impl)
macro_rules! Depcrate_processimpl_63 {
() => {
// Module: crate::process
// Provides: {"impl_63"}
// Dependencies: {}
impl std :: fmt :: Display for CommandError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . kind { CommandErrorKind :: Io (ref e) => e . fmt (f) , CommandErrorKind :: Stderr (ref bytes) => { let msg = String :: from_utf8_lossy (bytes) ; if msg . trim () . is_empty () { write ! (f , "<stderr is empty>") } else { let div = "-" . repeat (79) ; write ! (f , "\n{div}\n{msg}\n{div}" , div = div , msg = msg . trim ()) } } } } }
};
}
