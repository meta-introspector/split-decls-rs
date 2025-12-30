// Generated macro for impl_15 (impl)
macro_rules! Depcrate_compiled_keysimpl_15 {
() => {
// Module: crate::compiled_keys
// Provides: {"impl_15"}
// Dependencies: {}
impl fmt :: Display for CompileError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { CompileError :: AccountIndexOverflow => { f . write_str ("account index overflowed during compilation") } CompileError :: AddressTableLookupIndexOverflow => { f . write_str ("address lookup table index overflowed during compilation") } CompileError :: UnknownInstructionKey (key) => f . write_fmt (format_args ! ("encountered unknown account key `{key}` during instruction compilation" ,)) , } } }
};
}
