// Generated macro for sample_uniform (macro)
macro_rules! Depcrate_numsample_uniform {
() => {
// Module: crate::num
// Provides: {"sample_uniform"}
// Dependencies: {}
macro_rules ! sample_uniform { ($ name : ident , $ incl : ident , $ from : ty , $ to : ty) => { fn $ name < X > (run : & mut TestRunner , start : $ to , end : $ to ,) -> $ to { Uniform ::<$ from >:: new (start as $ from , end as $ from) . expect ("not uniform") . sample (run . rng ()) as $ to } fn $ incl < X > (run : & mut TestRunner , start : $ to , end : $ to ,) -> $ to { Uniform ::<$ from >:: new_inclusive (start as $ from , end as $ from) . expect ("not uniform") . sample (run . rng ()) as $ to } } }
};
}
