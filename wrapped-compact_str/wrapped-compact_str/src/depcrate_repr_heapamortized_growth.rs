// Generated macro for amortized_growth (function)
macro_rules! Depcrate_repr_heapamortized_growth {
() => {
// Module: crate::repr::heap
// Provides: {"amortized_growth"}
// Dependencies: {}
# [doc = " [`HeapBuffer`] grows at an amortized rates of 1.5x"] # [doc = ""] # [doc = " Note: this is different than [`std::string::String`], which grows at a rate of 2x. It's debated"] # [doc = " which is better, for now we'll stick with a rate of 1.5x"] # [inline (always)] pub (crate) fn amortized_growth (cur_len : usize , additional : usize) -> usize { let required = cur_len . saturating_add (additional) ; let amortized = cur_len . saturating_mul (3) / 2 ; amortized . max (required) }
};
}
