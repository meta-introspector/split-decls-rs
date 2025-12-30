// Generated wrapped crate with macro-based items

include!("depcrateadd_two.rs");
include!("depcrateinternal_adder.rs");
include!("depcratetests.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
