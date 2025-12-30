// Generated macro for InsertReuseBuilder (struct)
macro_rules! Depcrate_ir_builderInsertReuseBuilder {
() => {
// Module: crate::ir::builder
// Provides: {"InsertReuseBuilder"}
// Dependencies: {}
# [doc = " Builder that inserts a new instruction like `InsertBuilder`, but reusing result values."] pub struct InsertReuseBuilder < 'f , IIB , Array > where IIB : InstInserterBase < 'f > , Array : AsRef < [Option < Value >] > , { inserter : IIB , reuse : Array , unused : PhantomData < & 'f u32 > , }
};
}
