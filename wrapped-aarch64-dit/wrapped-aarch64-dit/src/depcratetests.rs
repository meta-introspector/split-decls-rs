// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { get_dit_enabled , restore_dit , set_dit_enabled , Dit } ; cpufeatures :: new ! (dit_supported , "dit") ; # [test] fn high_level_api () { let dit = Dit :: init () ; assert ! (dit . is_supported ()) ; { assert ! (! dit . is_enabled ()) ; let _guard = dit . enable () ; assert ! (dit . is_enabled ()) ; { let _guard2 = dit . enable () ; assert ! (dit . is_enabled ()) ; } assert ! (dit . is_enabled ()) ; } assert ! (! dit . is_enabled ()) ; } # [test] fn asm_wrappers () { let dit_token = dit_supported :: init () ; if ! dit_token . get () { panic ! ("DIT is not available on this CPU") ; } let dit_enabled = unsafe { get_dit_enabled () } ; assert ! (! dit_enabled) ; let was_enabled = unsafe { set_dit_enabled () } ; assert ! (! was_enabled) ; let dit_enabled = unsafe { get_dit_enabled () } ; assert ! (dit_enabled) ; unsafe { restore_dit (true) } ; let dit_enabled = unsafe { get_dit_enabled () } ; assert ! (dit_enabled) ; unsafe { restore_dit (false) } ; let dit_enabled = unsafe { get_dit_enabled () } ; assert ! (! dit_enabled) ; } }
};
}
