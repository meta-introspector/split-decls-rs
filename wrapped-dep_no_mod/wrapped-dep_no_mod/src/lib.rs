// Generated wrapped crate with macro-based items

include!("depcrate_foothing.rs");
include!("depcratefoo.rs");
include!("depcrate_foohello.rs");
include!("modcrate.rs");
include!("modcrate_foo.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_foo!();
}
