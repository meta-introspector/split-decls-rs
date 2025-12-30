// Generated wrapped crate with macro-based items

include!("depcratemain.rs");
include!("depcratefoo.rs");
include!("depcratebar.rs");
include!("depcratedo_stuff.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
