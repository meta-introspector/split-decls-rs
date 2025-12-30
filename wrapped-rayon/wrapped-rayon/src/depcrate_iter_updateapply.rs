// Generated macro for apply (function)
macro_rules! Depcrate_iter_updateapply {
() => {
// Module: crate::iter::update
// Provides: {"apply"}
// Dependencies: {}
fn apply < T > (update_op : impl Fn (& mut T)) -> impl Fn (T) -> T { move | mut item | { update_op (& mut item) ; item } }
};
}
