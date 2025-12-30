// Generated macro for impl_452 (impl)
macro_rules! Depcrate_validation_visitors_depthimpl_452 {
() => {
// Module: crate::validation::visitors::depth
// Provides: {"impl_452"}
// Dependencies: {}
impl < 'ctx > Visitor < 'ctx > for DepthCalculate < '_ > { fn mode (& self) -> VisitMode { VisitMode :: Inline } fn enter_field (& mut self , _ctx : & mut VisitorContext < 'ctx > , _field : & 'ctx Positioned < Field >) { self . current_depth += 1 ; * self . max_depth = (* self . max_depth) . max (self . current_depth) ; } fn exit_field (& mut self , _ctx : & mut VisitorContext < 'ctx > , _field : & 'ctx Positioned < Field >) { self . current_depth -= 1 ; } }
};
}
