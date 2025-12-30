// Generated macro for impl_192 (impl)
macro_rules! Depcrate_machinst_abiimpl_192 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_192"}
// Dependencies: {}
impl ABIArg { # [doc = " Create an ABIArg from one register."] pub fn reg (reg : RealReg , ty : ir :: Type , extension : ir :: ArgumentExtension , purpose : ir :: ArgumentPurpose ,) -> ABIArg { ABIArg :: Slots { slots : smallvec ! [ABIArgSlot :: Reg { reg , ty , extension }] , purpose , } } # [doc = " Create an ABIArg from one stack slot."] pub fn stack (offset : i64 , ty : ir :: Type , extension : ir :: ArgumentExtension , purpose : ir :: ArgumentPurpose ,) -> ABIArg { ABIArg :: Slots { slots : smallvec ! [ABIArgSlot :: Stack { offset , ty , extension , }] , purpose , } } }
};
}
