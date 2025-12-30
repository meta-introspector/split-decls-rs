// Generated wrapped crate with macro-based items

include!("depcratecfg_if.rs");
include!("depcratetests.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
