// Generated wrapped crate with macro-based items

include!("depcrateuse_1.rs");
include!("depcratemacro_2.rs");
include!("depcrateuse_the_dependency.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
