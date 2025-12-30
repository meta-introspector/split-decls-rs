// Generated macro for impl_262 (impl)
macro_rules! Depcrate_serimpl_262 {
() => {
// Module: crate::ser
// Provides: {"impl_262"}
// Dependencies: {}
impl < 'a > SeqSerializer < 'a > { fn new (inner : & 'a mut ConfigSerializer) -> Result < Self > { inner . keys . push (SerKey :: Seq (0)) ; Ok (SeqSerializer (inner)) } fn end (self) -> & 'a mut ConfigSerializer { let _ : Option < SerKey > = self . 0 . keys . pop () ; self . 0 } }
};
}
