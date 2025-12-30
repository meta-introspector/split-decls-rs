// Generated macro for make_type_and_const_binders (function)
macro_rules! Depcratemake_type_and_const_binders {
() => {
// Module: crate
// Provides: {"make_type_and_const_binders"}
// Dependencies: {}
pub (crate) fn make_type_and_const_binders < T : HasInterner < Interner = Interner > > (which_is_const : impl Iterator < Item = Option < Ty > > , value : T ,) -> Binders < T > { Binders :: new (VariableKinds :: from_iter (Interner , which_is_const . map (| x | { if let Some (ty) = x { chalk_ir :: VariableKind :: Const (ty) } else { chalk_ir :: VariableKind :: Ty (chalk_ir :: TyVariableKind :: General) } }) ,) , value ,) }
};
}
