// Generated wrapped crate with macro-based items

include!("depcrate_submodule_codetest.rs");
include!("depcratesubmodule.rs");
include!("depcrateother_1.rs");
include!("depcrate_submodulecode.rs");
include!("modcrate_submodule_code.rs");
include!("modcrate_submodule.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate_submodule_code!();
    Modcrate_submodule!();
    Modcrate!();
}
