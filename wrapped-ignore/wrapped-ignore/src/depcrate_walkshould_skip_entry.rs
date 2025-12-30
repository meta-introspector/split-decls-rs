// Generated macro for should_skip_entry (function)
macro_rules! Depcrate_walkshould_skip_entry {
() => {
// Module: crate::walk
// Provides: {"should_skip_entry"}
// Dependencies: {}
fn should_skip_entry (ig : & Ignore , dent : & DirEntry) -> bool { let m = ig . matched_dir_entry (dent) ; if m . is_ignore () { log :: debug ! ("ignoring {}: {:?}" , dent . path () . display () , m) ; true } else if m . is_whitelist () { log :: debug ! ("whitelisting {}: {:?}" , dent . path () . display () , m) ; false } else { false } }
};
}
