// Generated wrapped crate with macro-based items

include!("depcratefoo.rs");
include!("depcratecode_inlined.rs");
include!("depcratecode_generic.rs");
include!("depcratecode.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
