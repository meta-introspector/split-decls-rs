// Generated wrapped crate with macro-based items

include!("depcrate_otherfoo.rs");
include!("depcrate_foobar.rs");
include!("depcratefoo.rs");
include!("depcrateother.rs");
include!("modcrate_other_foo.rs");
include!("modcrate.rs");
include!("modcrate_other.rs");
include!("modcrate_foo.rs");

// Execute all items
pub fn execute_all() {
    Modcrate_other_foo!();
    Modcrate!();
    Modcrate_other!();
    Modcrate_foo!();
}
