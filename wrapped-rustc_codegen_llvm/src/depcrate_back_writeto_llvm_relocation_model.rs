// Generated macro for to_llvm_relocation_model (function)
macro_rules! Depcrate_back_writeto_llvm_relocation_model {
() => {
// Module: crate::back::write
// Provides: {"to_llvm_relocation_model"}
// Dependencies: {}
fn to_llvm_relocation_model (relocation_model : RelocModel) -> llvm :: RelocModel { match relocation_model { RelocModel :: Static => llvm :: RelocModel :: Static , RelocModel :: Pic | RelocModel :: Pie => llvm :: RelocModel :: PIC , RelocModel :: DynamicNoPic => llvm :: RelocModel :: DynamicNoPic , RelocModel :: Ropi => llvm :: RelocModel :: ROPI , RelocModel :: Rwpi => llvm :: RelocModel :: RWPI , RelocModel :: RopiRwpi => llvm :: RelocModel :: ROPI_RWPI , } }
};
}
