// Generated macro for impl_3865 (impl)
macro_rules! Depcrate_loops_utilsimpl_3865 {
() => {
// Module: crate::loops::utils
// Provides: {"impl_3865"}
// Dependencies: {}
impl < 'a , 'tcx > InitializeVisitor < 'a , 'tcx > { pub (super) fn new (cx : & 'a LateContext < 'tcx > , end_expr : & 'tcx Expr < 'tcx > , var_id : HirId) -> Self { Self { cx , end_expr , var_id , state : InitializeVisitorState :: Initial , depth : 0 , past_loop : false , } } pub (super) fn get_result (& self) -> Option < (Symbol , Option < Ty < 'tcx > > , & 'tcx Expr < 'tcx >) > { if let InitializeVisitorState :: Initialized { name , ty , initializer } = self . state { Some ((name , ty , initializer)) } else { None } } }
};
}
