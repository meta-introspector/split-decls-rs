// Generated wrapped crate with macro-based items

include!("depcrate_with_modthing.rs");
include!("depcratewith_mod.rs");
include!("depcrate_with_modinner.rs");
include!("depcrate_with_mod_innerstuff.rs");
include!("depcratefoo.rs");
include!("modcrate.rs");
include!("modcrate_with_mod.rs");
include!("modcrate_with_mod_inner.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_with_mod!();
    Modcrate_with_mod_inner!();
}
