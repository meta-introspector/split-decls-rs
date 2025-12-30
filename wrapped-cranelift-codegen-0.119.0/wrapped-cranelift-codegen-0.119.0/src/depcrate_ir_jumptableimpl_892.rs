// Generated macro for impl_892 (impl)
macro_rules! Depcrate_ir_jumptableimpl_892 {
() => {
// Module: crate::ir::jumptable
// Provides: {"impl_892"}
// Dependencies: {}
impl < 'a > Display for DisplayJumpTable < 'a > { fn fmt (& self , fmt : & mut Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{}, [" , self . jt . default_block () . display (self . pool)) ? ; if let Some ((first , rest)) = self . jt . as_slice () . split_first () { write ! (fmt , "{}" , first . display (self . pool)) ? ; for block in rest { write ! (fmt , ", {}" , block . display (self . pool)) ? ; } } write ! (fmt , "]") } }
};
}
