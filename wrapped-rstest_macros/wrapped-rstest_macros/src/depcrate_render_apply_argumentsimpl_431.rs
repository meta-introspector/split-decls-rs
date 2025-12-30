// Generated macro for impl_431 (impl)
macro_rules! Depcrate_render_apply_argumentsimpl_431 {
() => {
// Module: crate::render::apply_arguments
// Provides: {"impl_431"}
// Dependencies: {}
impl ApplyArguments for FnArg { type Output = Option < Lifetime > ; type Context = usize ; fn apply_arguments (& mut self , arguments : & mut ArgumentsInfo , anoymous_id : & mut usize ,) -> Self :: Output { if self . maybe_pat () . map (| id | arguments . is_future (id)) . unwrap_or_default () { self . impl_future_arg (anoymous_id) } else { None } } }
};
}
