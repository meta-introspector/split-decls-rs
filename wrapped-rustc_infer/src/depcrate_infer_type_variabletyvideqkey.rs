// Generated macro for TyVidEqKey (struct)
macro_rules! Depcrate_infer_type_variableTyVidEqKey {
() => {
// Module: crate::infer::type_variable
// Provides: {"TyVidEqKey"}
// Dependencies: {}
# [doc = " These structs (a newtyped TyVid) are used as the unification key"] # [doc = " for the `eq_relations`; they carry a `TypeVariableValue` along"] # [doc = " with them."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct TyVidEqKey < 'tcx > { vid : ty :: TyVid , phantom : PhantomData < TypeVariableValue < 'tcx > > , }
};
}
