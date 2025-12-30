// Generated macro for make_single_type_binders (function)
macro_rules! Depcratemake_single_type_binders {
() => {
// Module: crate
// Provides: {"make_single_type_binders"}
// Dependencies: {}
pub (crate) fn make_single_type_binders < T : HasInterner < Interner = Interner > > (value : T ,) -> Binders < T > { Binders :: new (VariableKinds :: from_iter (Interner , std :: iter :: once (chalk_ir :: VariableKind :: Ty (chalk_ir :: TyVariableKind :: General)) ,) , value ,) }
};
}
