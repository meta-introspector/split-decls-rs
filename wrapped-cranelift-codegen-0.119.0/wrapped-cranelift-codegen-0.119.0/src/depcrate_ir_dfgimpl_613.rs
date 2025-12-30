// Generated macro for impl_613 (impl)
macro_rules! Depcrate_ir_dfgimpl_613 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_613"}
// Dependencies: {}
impl From < ValueData > for ValueDataPacked { fn from (data : ValueData) -> Self { match data { ValueData :: Inst { ty , num , inst } => { Self :: make (Self :: TAG_INST , ty , num . into () , inst . as_bits ()) } ValueData :: Param { ty , num , block } => { Self :: make (Self :: TAG_PARAM , ty , num . into () , block . as_bits ()) } ValueData :: Alias { ty , original } => { Self :: make (Self :: TAG_ALIAS , ty , 0 , original . as_bits ()) } ValueData :: Union { ty , x , y } => { Self :: make (Self :: TAG_UNION , ty , x . as_bits () , y . as_bits ()) } } } }
};
}
