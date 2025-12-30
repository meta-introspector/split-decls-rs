// Generated macro for impl_33 (impl)
macro_rules! Depcrate_navigation_targetimpl_33 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_33"}
// Dependencies: {}
impl < T : TryToNav , U : TryToNav > TryToNav for Either < T , U > { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { Either :: Left (it) => it . try_to_nav (sema) , Either :: Right (it) => it . try_to_nav (sema) , } } }
};
}
