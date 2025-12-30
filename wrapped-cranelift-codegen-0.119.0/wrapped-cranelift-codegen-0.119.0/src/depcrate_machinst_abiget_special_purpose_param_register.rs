// Generated macro for get_special_purpose_param_register (function)
macro_rules! Depcrate_machinst_abiget_special_purpose_param_register {
() => {
// Module: crate::machinst::abi
// Provides: {"get_special_purpose_param_register"}
// Dependencies: {}
fn get_special_purpose_param_register (f : & ir :: Function , sigs : & SigSet , sig : Sig , purpose : ir :: ArgumentPurpose ,) -> Option < Reg > { let idx = f . signature . special_param_index (purpose) ? ; match & sigs . args (sig) [idx] { & ABIArg :: Slots { ref slots , .. } => match & slots [0] { & ABIArgSlot :: Reg { reg , .. } => Some (reg . into ()) , _ => None , } , _ => None , } }
};
}
