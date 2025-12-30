// Generated macro for impl_31 (impl)
macro_rules! Depcrate_navigation_targetimpl_31 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : TryToNav , U : TryToNav > TryToNav for Either < T , U > { fn try_to_nav (& self , db : & RootDatabase) -> Option < UpmappingResult < NavigationTarget > > { match self { Either :: Left (it) => it . try_to_nav (db) , Either :: Right (it) => it . try_to_nav (db) , } } }
};
}
