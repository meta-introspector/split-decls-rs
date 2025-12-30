// Generated macro for feature_switch (macro)
macro_rules! Depcratefeature_switch {
() => {
// Module: crate
// Provides: {"feature_switch"}
// Dependencies: {}
macro_rules ! feature_switch { (($ feature : literal => $ with : path | $ without : path) ($ ($ args : tt) *)) => { # [cfg (feature = $ feature)] $ with ! ($ ($ args) *) ; # [cfg (not (feature = $ feature))] $ without ! ($ ($ args) *) ; } ; }
};
}
