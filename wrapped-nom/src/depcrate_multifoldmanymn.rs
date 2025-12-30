// Generated macro for FoldManyMN (struct)
macro_rules! Depcrate_multiFoldManyMN {
() => {
// Module: crate::multi
// Provides: {"FoldManyMN"}
// Dependencies: {}
# [doc = " Parser implementation for the [fold_many_m_n] combinator"] pub struct FoldManyMN < F , G , Init , R > { parser : F , g : G , init : Init , r : PhantomData < R > , min : usize , max : usize , }
};
}
