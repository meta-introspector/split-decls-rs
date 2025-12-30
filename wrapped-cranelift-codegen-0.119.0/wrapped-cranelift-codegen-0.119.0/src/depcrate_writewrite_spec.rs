// Generated macro for write_spec (function)
macro_rules! Depcrate_writewrite_spec {
() => {
// Module: crate::write
// Provides: {"write_spec"}
// Dependencies: {}
fn write_spec (w : & mut dyn Write , func : & Function) -> fmt :: Result { write ! (w , "{}{}" , func . name , func . signature) }
};
}
