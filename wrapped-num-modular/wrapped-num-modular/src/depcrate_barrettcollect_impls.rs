// Generated macro for collect_impls (macro)
macro_rules! Depcrate_barrettcollect_impls {
() => {
// Module: crate::barrett
// Provides: {"collect_impls"}
// Dependencies: {}
macro_rules ! collect_impls { ($ T : ident , $ ns : ident) => { mod $ ns { use super ::*; use crate :: word ::$ T ::*; impl_premulinv_1by1_for ! (Word) ; impl_normdiv_2by1_for ! (Word , DoubleWord) ; impl_premulinv_2by1_reducer_for ! (Word) ; impl_normdiv_3by2_for ! (Word , DoubleWord) ; impl_premulinv_3by2_reducer_for ! (Word , DoubleWord) ; } } ; }
};
}
