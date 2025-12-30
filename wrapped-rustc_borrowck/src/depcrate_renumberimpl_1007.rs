// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_renumberimpl_1007 {
() => {
// Module: crate::renumber
// Provides: {"impl_1007"}
// Dependencies: {}
impl < 'a , 'tcx > MutVisitor < 'tcx > for RegionRenumberer < 'a , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } # [instrument (skip (self) , level = "debug")] fn visit_ty (& mut self , ty : & mut Ty < 'tcx > , ty_context : TyContext) { if matches ! (ty_context , TyContext :: ReturnTy (_)) { return ; } * ty = self . renumber_regions (* ty , | | RegionCtxt :: TyContext (ty_context)) ; debug ! (? ty) ; } # [instrument (skip (self) , level = "debug")] fn visit_args (& mut self , args : & mut GenericArgsRef < 'tcx > , location : Location) { * args = self . renumber_regions (* args , | | RegionCtxt :: Location (location)) ; debug ! (? args) ; } # [instrument (skip (self) , level = "debug")] fn visit_region (& mut self , region : & mut ty :: Region < 'tcx > , location : Location) { let old_region = * region ; * region = self . renumber_regions (old_region , | | RegionCtxt :: Location (location)) ; debug ! (? region) ; } # [instrument (skip (self) , level = "debug")] fn visit_ty_const (& mut self , ct : & mut ty :: Const < 'tcx > , location : Location) { let old_ct = * ct ; * ct = self . renumber_regions (old_ct , | | RegionCtxt :: Location (location)) ; debug ! (? ct) ; } # [instrument (skip (self) , level = "debug")] fn visit_const_operand (& mut self , constant : & mut ConstOperand < 'tcx > , location : Location) { let const_ = constant . const_ ; constant . const_ = self . renumber_regions (const_ , | | RegionCtxt :: Location (location)) ; debug ! ("constant: {:#?}" , constant) ; } }
};
}
