// Generated wrapped crate with macro-based items

include!("depcratetest2.rs");
include!("depcratetest1.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
