// Generated macro for impl_384 (impl)
macro_rules! Depcrate_rowimpl_384 {
() => {
// Module: crate::row
// Provides: {"impl_384"}
// Dependencies: {}
impl < 'stmt > Rows < 'stmt > { # [inline] pub (crate) fn new (stmt : & 'stmt Statement < 'stmt >) -> Self { Rows { stmt : Some (stmt) , row : None , } } # [inline] pub (crate) fn get_expected_row (& mut self) -> Result < & Row < 'stmt > > { match self . next () ? { Some (row) => Ok (row) , None => Err (Error :: QueryReturnedNoRows) , } } }
};
}
