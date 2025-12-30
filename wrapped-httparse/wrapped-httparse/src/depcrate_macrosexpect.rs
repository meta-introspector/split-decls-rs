// Generated macro for expect (macro)
macro_rules! Depcrate_macrosexpect {
() => {
// Module: crate::macros
// Provides: {"expect"}
// Dependencies: {}
macro_rules ! expect { ($ bytes : ident . next () == $ pat : pat_param => $ ret : expr) => { expect ! (next ! ($ bytes) => $ pat |? $ ret) } ; ($ e : expr => $ pat : pat_param |? $ ret : expr) => { match $ e { v @$ pat => v , _ => return $ ret } } ; }
};
}
