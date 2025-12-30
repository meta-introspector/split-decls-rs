// Generated wrapped crate with macro-based items

include!("depcrate_badthing.rs");
include!("depcratebad.rs");
include!("depcratemain.rs");
include!("modcrate.rs");
include!("modcrate_bad.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_bad!();
}
