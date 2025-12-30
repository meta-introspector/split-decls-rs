// Generated macro for impl_434 (impl)
macro_rules! Depcrate_render_apply_argumentsimpl_434 {
() => {
// Module: crate::render::apply_arguments
// Provides: {"impl_434"}
// Dependencies: {}
impl ApplyArguments for Signature { type Output = () ; type Context = () ; fn apply_arguments (& mut self , arguments : & mut ArgumentsInfo , _ : & mut ()) { let mut anonymous_lt = 0_usize ; let new_lifetimes = self . inputs . iter_mut () . filter_map (| arg | arg . apply_arguments (arguments , & mut anonymous_lt)) . collect :: < Vec < _ > > () ; if ! new_lifetimes . is_empty () || ! self . generics . params . is_empty () { let new_generics = extend_generics_with_lifetimes (self . generics . params . iter () , new_lifetimes . iter ()) ; move_generic_list (& mut self . generics , new_generics) ; } } }
};
}
