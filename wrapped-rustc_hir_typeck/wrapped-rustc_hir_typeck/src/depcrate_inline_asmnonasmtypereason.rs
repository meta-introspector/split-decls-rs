// Generated macro for NonAsmTypeReason (enum)
macro_rules! Depcrate_inline_asmNonAsmTypeReason {
() => {
// Module: crate::inline_asm
// Provides: {"NonAsmTypeReason"}
// Dependencies: {}
enum NonAsmTypeReason < 'tcx > { UnevaluatedSIMDArrayLength (DefId , ty :: Const < 'tcx >) , Invalid (Ty < 'tcx >) , InvalidElement (DefId , Ty < 'tcx >) , NotSizedPtr (Ty < 'tcx >) , EmptySIMDArray (Ty < 'tcx >) , }
};
}
