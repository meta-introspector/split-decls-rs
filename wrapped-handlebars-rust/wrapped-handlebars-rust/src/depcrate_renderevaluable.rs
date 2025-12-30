// Generated macro for Evaluable (trait)
macro_rules! Depcrate_renderEvaluable {
() => {
// Module: crate::render
// Provides: {"Evaluable"}
// Dependencies: {}
# [doc = " Evaluate decorator"] pub trait Evaluable { fn eval < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , context : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < () , RenderError > ; }
};
}
