// Generated wrapped crate with macro-based items

include!("depcrate__private.rs");
include!("depcratestack_pin.rs");
include!("depcrate_stack_pinpin_mut.rs");
include!("depcrate_projectionunsafe_unpinned.rs");
include!("depcrate_projectionunsafe_pinned.rs");
include!("depcrateprojection.rs");
include!("modcrate.rs");
include!("modcrate_stack_pin.rs");
include!("modcrate_projection.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_stack_pin!();
    Modcrate_projection!();
}
