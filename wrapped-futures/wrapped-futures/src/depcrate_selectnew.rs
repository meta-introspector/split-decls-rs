// Generated macro for new (function)
macro_rules! Depcrate_selectnew {
() => {
// Module: crate::select
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , B > (a : A , b : B) -> Select < A , B > where A : Future , B : Future < Item = A :: Item , Error = A :: Error > { let a = Collapsed :: Start (a) ; let b = Collapsed :: Start (b) ; Select { inner : Some ((a , b)) , } }
};
}
