// Generated macro for clear_if_dirty (function)
macro_rules! Depcrate_utils_build_stampclear_if_dirty {
() => {
// Module: crate::utils::build_stamp
// Provides: {"clear_if_dirty"}
// Dependencies: {}
# [doc = " Clear out `dir` if `input` is newer."] # [doc = ""] # [doc = " After this executes, it will also ensure that `dir` exists."] pub fn clear_if_dirty (builder : & Builder < '_ > , dir : & Path , input : & Path) -> bool { let stamp = BuildStamp :: new (dir) ; let mut cleared = false ; if mtime (stamp . path ()) < mtime (input) { builder . verbose (| | println ! ("Dirty - {}" , dir . display ())) ; let _ = fs :: remove_dir_all (dir) ; cleared = true ; } else if stamp . path () . exists () { return cleared ; } t ! (fs :: create_dir_all (dir)) ; t ! (fs :: File :: create (stamp . path ())) ; cleared }
};
}
