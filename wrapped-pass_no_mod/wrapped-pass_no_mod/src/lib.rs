// Generated wrapped crate with macro-based items

include!("depcratemain.rs");
include!("depcrategood.rs");
include!("depcrate_goodthing.rs");
include!("modcrate.rs");
include!("modcrate_good.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_good!();
}
