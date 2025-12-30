// Generated macro for both (function)
macro_rules! Depcrate_hir_utilsboth {
() => {
// Module: crate::hir_utils
// Provides: {"both"}
// Dependencies: {}
# [doc = " Checks if the two `Option`s are both `None` or some equal values as per"] # [doc = " `eq_fn`."] pub fn both < X > (l : Option < & X > , r : Option < & X > , mut eq_fn : impl FnMut (& X , & X) -> bool) -> bool { l . as_ref () . map_or_else (| | r . is_none () , | x | r . as_ref () . is_some_and (| y | eq_fn (x , y))) }
};
}
