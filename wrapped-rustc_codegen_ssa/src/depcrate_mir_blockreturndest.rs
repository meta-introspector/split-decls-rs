// Generated macro for ReturnDest (enum)
macro_rules! Depcrate_mir_blockReturnDest {
() => {
// Module: crate::mir::block
// Provides: {"ReturnDest"}
// Dependencies: {}
enum ReturnDest < 'tcx , V > { # [doc = " Do nothing; the return value is indirect or ignored."] Nothing , # [doc = " Store the return value to the pointer."] Store (PlaceRef < 'tcx , V >) , # [doc = " Store an indirect return value to an operand local place."] IndirectOperand (PlaceRef < 'tcx , V > , mir :: Local) , # [doc = " Store a direct return value to an operand local place."] DirectOperand (mir :: Local) , }
};
}
