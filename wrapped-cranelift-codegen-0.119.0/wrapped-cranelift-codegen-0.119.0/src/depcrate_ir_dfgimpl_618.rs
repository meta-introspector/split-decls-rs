// Generated macro for impl_618 (impl)
macro_rules! Depcrate_ir_dfgimpl_618 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_618"}
// Dependencies: {}
impl BlockData { fn new () -> Self { Self { params : ValueList :: new () , } } # [doc = " Get the parameters on `block`."] pub fn params < 'a > (& self , pool : & 'a ValueListPool) -> & 'a [Value] { self . params . as_slice (pool) } }
};
}
