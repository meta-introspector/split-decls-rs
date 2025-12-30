// Generated macro for impl_43 (impl)
macro_rules! Depcrate_writer_targetimpl_43 {
() => {
// Module: crate::writer::target
// Provides: {"impl_43"}
// Dependencies: {}
impl std :: fmt :: Debug for Target { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , match self { Self :: Stdout => "stdout" , Self :: Stderr => "stderr" , Self :: Pipe (_) => "pipe" , }) } }
};
}
