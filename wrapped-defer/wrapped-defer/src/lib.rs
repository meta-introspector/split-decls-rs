// Generated wrapped crate with macro-based items

include!("depcratetest.rs");
include!("depcrateuse_1.rs");
include!("depcratetest_macro.rs");
include!("depcratedefer.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
