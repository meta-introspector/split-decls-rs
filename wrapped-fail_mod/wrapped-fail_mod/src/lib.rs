// Generated wrapped crate with macro-based items

include!("depcratemain.rs");
include!("depcrate_bad_innerstuff.rs");
include!("depcrate_badthing.rs");
include!("depcrate_badinner.rs");
include!("depcratebad.rs");
include!("modcrate.rs");
include!("modcrate_bad_inner.rs");
include!("modcrate_bad.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_bad_inner!();
    Modcrate_bad!();
}
