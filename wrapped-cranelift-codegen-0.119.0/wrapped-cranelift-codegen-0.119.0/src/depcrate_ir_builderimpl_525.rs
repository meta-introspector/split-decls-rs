// Generated macro for impl_525 (impl)
macro_rules! Depcrate_ir_builderimpl_525 {
() => {
// Module: crate::ir::builder
// Provides: {"impl_525"}
// Dependencies: {}
impl < 'f , IIB : InstInserterBase < 'f > > InsertBuilder < 'f , IIB > { # [doc = " Create a new builder which inserts instructions at `pos`."] # [doc = " The `dfg` and `pos.layout` references should be from the same `Function`."] pub fn new (inserter : IIB) -> Self { Self { inserter , unused : PhantomData , } } # [doc = " Reuse result values in `reuse`."] # [doc = ""] # [doc = " Convert this builder into one that will reuse the provided result values instead of"] # [doc = " allocating new ones. The provided values for reuse must not be attached to anything. Any"] # [doc = " missing result values will be allocated as normal."] # [doc = ""] # [doc = " The `reuse` argument is expected to be an array of `Option<Value>`."] pub fn with_results < Array > (self , reuse : Array) -> InsertReuseBuilder < 'f , IIB , Array > where Array : AsRef < [Option < Value >] > , { InsertReuseBuilder { inserter : self . inserter , reuse , unused : PhantomData , } } # [doc = " Reuse a single result value."] # [doc = ""] # [doc = " Convert this into a builder that will reuse `v` as the single result value. The reused"] # [doc = " result value `v` must not be attached to anything."] # [doc = ""] # [doc = " This method should only be used when building an instruction with exactly one result. Use"] # [doc = " `with_results()` for the more general case."] pub fn with_result (self , v : Value) -> InsertReuseBuilder < 'f , IIB , [Option < Value > ; 1] > { self . with_results ([Some (v)]) } }
};
}
