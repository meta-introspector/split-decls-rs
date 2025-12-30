// Generated macro for Operand (enum)
macro_rules! Depcrate_interpret_operandOperand {
() => {
// Module: crate::interpret::operand
// Provides: {"Operand"}
// Dependencies: {}
# [doc = " An `Operand` is the result of computing a `mir::Operand`. It can be immediate,"] # [doc = " or still in memory. The latter is an optimization, to delay reading that chunk of"] # [doc = " memory and to avoid having to store arbitrary-sized data here."] # [derive (Copy , Clone , Debug)] pub (super) enum Operand < Prov : Provenance = CtfeProvenance > { Immediate (Immediate < Prov >) , Indirect (MemPlace < Prov >) , }
};
}
