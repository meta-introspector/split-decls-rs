// Generated wrapped crate with macro-based items

include!("depcratemain.rs");
include!("depcratedo_twice.rs");
include!("depcrateadd_one.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
