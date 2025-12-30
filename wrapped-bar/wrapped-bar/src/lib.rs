// Generated wrapped crate with macro-based items

include!("depcratefoo.rs");
include!("modcrate_foo.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate_foo!();
    Modcrate!();
}
