// Generated macro for impl_1016 (impl)
macro_rules! Depcrateimpl_1016 {
() => {
// Module: crate
// Provides: {"impl_1016"}
// Dependencies: {}
impl < 'conn > fallible_iterator :: FallibleIterator for Batch < 'conn , '_ > { type Error = Error ; type Item = Statement < 'conn > ; # [doc = " Iterates on each batch statements."] # [doc = ""] # [doc = " Returns `Ok(None)` when batch is completed."] fn next (& mut self) -> Result < Option < Statement < 'conn > > > { while self . tail < self . sql . len () { let sql = & self . sql [self . tail ..] ; let (next , tail) = self . conn . db . borrow_mut () . prepare (self . conn , sql , PrepFlags :: default ()) ? ; if tail == 0 { self . tail = self . sql . len () ; } else { self . tail += tail ; } if next . stmt . is_null () { continue ; } return Ok (Some (next)) ; } Ok (None) } }
};
}
