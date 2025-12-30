// Generated wrapped crate with macro-based items

include!("depcrate_more_innerinner.rs");
include!("depcrate_badthing.rs");
include!("depcratemore.rs");
include!("depcrate_more_foofoo.rs");
include!("depcrate_moreinner.rs");
include!("depcratebad.rs");
include!("depcrate_morefoo.rs");
include!("depcratemain.rs");
include!("modcrate.rs");
include!("modcrate_more.rs");
include!("modcrate_more_foo.rs");
include!("modcrate_more_inner.rs");
include!("modcrate_bad.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_more!();
    Modcrate_more_foo!();
    Modcrate_more_inner!();
    Modcrate_bad!();
}
