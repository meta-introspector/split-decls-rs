// Generated macro for impl_44 (impl)
macro_rules! Depcrate_astimpl_44 {
() => {
// Module: crate::ast
// Provides: {"impl_44"}
// Dependencies: {}
impl Strategy { fn types (& self) -> Vec < syn :: Type > { use self :: Strategy :: * ; match self { Arbitrary (ty , _) => vec ! [ty . clone ()] , Regex (ty) => vec ! [ty . clone ()] , Existential (ty) => vec ! [ty . clone ()] , Value (ty) => vec ! [ty . clone ()] , Map (strats) => strats . iter () . flat_map (| s | s . types ()) . collect () , Union (strats) => strats . iter () . flat_map (| s | s . types ()) . collect () , Filter (_ , ty) => vec ! [ty . clone ()] , } } }
};
}
