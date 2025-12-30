// Generated macro for flatten_ok (function)
macro_rules! Depcrate_flatten_okflatten_ok {
() => {
// Module: crate::flatten_ok
// Provides: {"flatten_ok"}
// Dependencies: {}
pub fn flatten_ok < I , T , E > (iter : I) -> FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > , T : IntoIterator , { FlattenOk { iter , inner_front : None , inner_back : None , } }
};
}
