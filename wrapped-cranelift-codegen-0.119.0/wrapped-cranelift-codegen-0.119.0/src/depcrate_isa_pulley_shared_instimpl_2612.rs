// Generated macro for impl_2612 (impl)
macro_rules! Depcrate_isa_pulley_shared_instimpl_2612 {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"impl_2612"}
// Dependencies: {}
impl Inst { # [doc = " Generic constructor for a load (zero-extending where appropriate)."] pub fn gen_load (dst : Writable < Reg > , mem : Amode , ty : Type , flags : MemFlags) -> Inst { if ty . is_vector () { assert_eq ! (ty . bytes () , 16) ; Inst :: VLoad { dst : dst . map (| r | VReg :: new (r) . unwrap ()) , mem , ty , flags , } } else if ty . is_int () { assert ! (ty . bytes () <= 8) ; Inst :: XLoad { dst : dst . map (| r | XReg :: new (r) . unwrap ()) , mem , ty , flags , } } else { Inst :: FLoad { dst : dst . map (| r | FReg :: new (r) . unwrap ()) , mem , ty , flags , } } } # [doc = " Generic constructor for a store."] pub fn gen_store (mem : Amode , from_reg : Reg , ty : Type , flags : MemFlags) -> Inst { if ty . is_vector () { assert_eq ! (ty . bytes () , 16) ; Inst :: VStore { mem , src : VReg :: new (from_reg) . unwrap () , ty , flags , } } else if ty . is_int () { assert ! (ty . bytes () <= 8) ; Inst :: XStore { mem , src : XReg :: new (from_reg) . unwrap () , ty , flags , } } else { Inst :: FStore { mem , src : FReg :: new (from_reg) . unwrap () , ty , flags , } } } }
};
}
