// Generated macro for InsertBuilder (struct)
macro_rules! Depcrate_ir_builderInsertBuilder {
() => {
// Module: crate::ir::builder
// Provides: {"InsertBuilder"}
// Dependencies: {}
# [doc = " Builder that inserts an instruction at the current position."] # [doc = ""] # [doc = " An `InsertBuilder` is a wrapper for an `InstInserterBase` that turns it into an instruction"] # [doc = " builder with some additional facilities for creating instructions that reuse existing values as"] # [doc = " their results."] pub struct InsertBuilder < 'f , IIB : InstInserterBase < 'f > > { inserter : IIB , unused : PhantomData < & 'f u32 > , }
};
}
