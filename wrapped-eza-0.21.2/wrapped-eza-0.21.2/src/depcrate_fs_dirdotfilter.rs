// Generated macro for DotFilter (enum)
macro_rules! Depcrate_fs_dirDotFilter {
() => {
// Module: crate::fs::dir
// Provides: {"DotFilter"}
// Dependencies: {}
# [doc = " Usually files in Unix use a leading dot to be hidden or visible, but two"] # [doc = " entries in particular are “extra-hidden”: `.` and `..`, which only become"] # [doc = " visible after an extra `-a` option."] # [derive (PartialEq , Eq , Debug , Default , Copy , Clone)] pub enum DotFilter { # [doc = " Shows files, dotfiles, and `.` and `..`."] DotfilesAndDots , # [doc = " Show files and dotfiles, but hide `.` and `..`."] Dotfiles , # [doc = " Just show files, hiding anything beginning with a dot."] # [default] JustFiles , }
};
}
