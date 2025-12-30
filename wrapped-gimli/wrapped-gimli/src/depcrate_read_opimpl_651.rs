// Generated macro for impl_651 (impl)
macro_rules! Depcrate_read_opimpl_651 {
() => {
// Module: crate::read::op
// Provides: {"impl_651"}
// Dependencies: {}
impl < R : Reader > OperationIter < R > { # [doc = " Read the next operation in an expression."] pub fn next (& mut self) -> Result < Option < Operation < R > > > { if self . input . is_empty () { return Ok (None) ; } match Operation :: parse (& mut self . input , self . encoding) { Ok (op) => Ok (Some (op)) , Err (e) => { self . input . empty () ; Err (e) } } } # [doc = " Return the current byte offset of the iterator."] pub fn offset_from (& self , expression : & Expression < R >) -> R :: Offset { self . input . offset_from (& expression . 0) } }
};
}
