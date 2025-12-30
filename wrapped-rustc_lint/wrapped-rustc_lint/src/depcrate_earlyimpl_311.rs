// Generated macro for impl_311 (impl)
macro_rules! Depcrate_earlyimpl_311 {
() => {
// Module: crate::early
// Provides: {"impl_311"}
// Dependencies: {}
impl < 'a > EarlyCheckNode < 'a > for (ast :: NodeId , & 'a [ast :: Attribute] , & 'a [Box < ast :: Item >]) { fn id (self) -> ast :: NodeId { self . 0 } fn attrs (self) -> & 'a [ast :: Attribute] { self . 1 } fn check < 'ecx , 'tcx , T : EarlyLintPass > (self , cx : & mut EarlyContextAndPass < 'ecx , 'tcx , T >) { walk_list ! (cx , visit_attribute , self . 1) ; walk_list ! (cx , visit_item , self . 2) ; } }
};
}
