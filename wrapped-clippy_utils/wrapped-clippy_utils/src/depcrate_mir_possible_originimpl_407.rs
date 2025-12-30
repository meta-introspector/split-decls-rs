// Generated macro for impl_407 (impl)
macro_rules! Depcrate_mir_possible_originimpl_407 {
() => {
// Module: crate::mir::possible_origin
// Provides: {"impl_407"}
// Dependencies: {}
impl < 'tcx > mir :: visit :: Visitor < 'tcx > for PossibleOriginVisitor < '_ , 'tcx > { fn visit_assign (& mut self , place : & mir :: Place < 'tcx > , rvalue : & mir :: Rvalue < '_ > , _location : mir :: Location) { let lhs = place . local ; match rvalue { mir :: Rvalue :: Ref (_ , mir :: BorrowKind :: Mut { .. } , borrowed) | mir :: Rvalue :: Use (mir :: Operand :: Move (borrowed)) | mir :: Rvalue :: Cast (_ , mir :: Operand :: Move (borrowed) , _) => { self . possible_origin . add (lhs , borrowed . local) ; } , _ => { } , } } }
};
}
