// Generated macro for move_generic_list (function)
macro_rules! Depcrate_render_apply_argumentsmove_generic_list {
() => {
// Module: crate::render::apply_arguments
// Provides: {"move_generic_list"}
// Dependencies: {}
fn move_generic_list (data : & mut Generics , other : Generics) { data . lt_token = data . lt_token . or (other . lt_token) ; data . params = other . params ; data . gt_token = data . gt_token . or (other . gt_token) ; }
};
}
