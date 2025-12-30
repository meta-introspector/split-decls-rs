// Generated macro for impl_354 (impl)
macro_rules! Depcrate_machinst_regimpl_354 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_354"}
// Dependencies: {}
impl std :: fmt :: Debug for Reg { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { if self . 0 == VReg :: invalid () { write ! (f , "<invalid>") } else if let Some (rreg) = self . to_real_reg () { let preg : PReg = rreg . into () ; write ! (f , "{preg}") } else if let Some (vreg) = self . to_virtual_reg () { let vreg : VReg = vreg . into () ; write ! (f , "{vreg}") } else { unreachable ! () } } }
};
}
