// Generated wrapped crate with macro-based items

include!("depcratemain.rs");
include!("depcratebuild_user.rs");
include!("depcrateuser.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
