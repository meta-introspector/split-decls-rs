// Generated macro for check_validity_requirement (function)
macro_rules! Depcrate_util_check_validity_requirementcheck_validity_requirement {
() => {
// Module: crate::util::check_validity_requirement
// Provides: {"check_validity_requirement"}
// Dependencies: {}
# [doc = " Determines if this type permits \"raw\" initialization by just transmuting some memory into an"] # [doc = " instance of `T`."] # [doc = ""] # [doc = " `init_kind` indicates if the memory is zero-initialized or left uninitialized. We assume"] # [doc = " uninitialized memory is mitigated by filling it with 0x01, which reduces the chance of causing"] # [doc = " LLVM UB."] # [doc = ""] # [doc = " By default we check whether that operation would cause *LLVM UB*, i.e., whether the LLVM IR we"] # [doc = " generate has UB or not. This is a mitigation strategy, which is why we are okay with accepting"] # [doc = " Rust UB as long as there is no risk of miscompilations. The `strict_init_checks` can be set to"] # [doc = " do a full check against Rust UB instead (in which case we will also ignore the 0x01-filling and"] # [doc = " to the full uninit check)."] pub fn check_validity_requirement < 'tcx > (tcx : TyCtxt < 'tcx > , kind : ValidityRequirement , input : PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> Result < bool , & 'tcx LayoutError < 'tcx > > { let layout = tcx . layout_of (input) ? ; if kind == ValidityRequirement :: Inhabited { return Ok (! layout . is_uninhabited ()) ; } let layout_cx = LayoutCx :: new (tcx , input . typing_env) ; if kind == ValidityRequirement :: Uninit || tcx . sess . opts . unstable_opts . strict_init_checks { Ok (check_validity_requirement_strict (layout , & layout_cx , kind)) } else { check_validity_requirement_lax (layout , & layout_cx , kind) } }
};
}
