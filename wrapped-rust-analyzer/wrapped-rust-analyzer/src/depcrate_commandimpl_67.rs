// Generated macro for impl_67 (impl)
macro_rules! Depcrate_commandimpl_67 {
() => {
// Module: crate::command
// Provides: {"impl_67"}
// Dependencies: {}
impl < T > fmt :: Debug for CommandHandle < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CommandHandle") . field ("program" , & self . program) . field ("arguments" , & self . arguments) . field ("current_dir" , & self . current_dir) . finish () } }
};
}
