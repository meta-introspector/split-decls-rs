// Generated macro for fold_tys (function)
macro_rules! Depcratefold_tys {
() => {
// Module: crate
// Provides: {"fold_tys"}
// Dependencies: {}
pub (crate) fn fold_tys < T : HasInterner < Interner = Interner > + TypeFoldable < Interner > > (t : T , mut for_ty : impl FnMut (Ty , DebruijnIndex) -> Ty , binders : DebruijnIndex ,) -> T { fold_tys_and_consts (t , | x , d | match x { Either :: Left (x) => Either :: Left (for_ty (x , d)) , Either :: Right (x) => Either :: Right (x) , } , binders ,) }
};
}
