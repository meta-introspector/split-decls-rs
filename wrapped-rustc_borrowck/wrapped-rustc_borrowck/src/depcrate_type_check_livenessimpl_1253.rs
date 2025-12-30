// Generated macro for impl_1253 (impl)
macro_rules! Depcrate_type_check_livenessimpl_1253 {
() => {
// Module: crate::type_check::liveness
// Provides: {"impl_1253"}
// Dependencies: {}
impl < 'a , 'tcx > Visitor < 'tcx > for LiveVariablesVisitor < 'a , 'tcx > { # [doc = " We sometimes have `args` within an rvalue, or within a"] # [doc = " call. Make them live at the location where they appear."] fn visit_args (& mut self , args : & GenericArgsRef < 'tcx > , location : Location) { self . record_regions_live_at (* args , location) ; self . super_args (args) ; } # [doc = " We sometimes have `region`s within an rvalue, or within a"] # [doc = " call. Make them live at the location where they appear."] fn visit_region (& mut self , region : Region < 'tcx > , location : Location) { self . record_regions_live_at (region , location) ; self . super_region (region) ; } # [doc = " We sometimes have `ty`s within an rvalue, or within a"] # [doc = " call. Make them live at the location where they appear."] fn visit_ty (& mut self , ty : Ty < 'tcx > , ty_context : TyContext) { match ty_context { TyContext :: ReturnTy (SourceInfo { span , .. }) | TyContext :: YieldTy (SourceInfo { span , .. }) | TyContext :: ResumeTy (SourceInfo { span , .. }) | TyContext :: UserTy (span) | TyContext :: LocalDecl { source_info : SourceInfo { span , .. } , .. } => { span_bug ! (span , "should not be visiting outside of the CFG: {:?}" , ty_context) ; } TyContext :: Location (location) => { self . record_regions_live_at (ty , location) ; } } self . super_ty (ty) ; } }
};
}
