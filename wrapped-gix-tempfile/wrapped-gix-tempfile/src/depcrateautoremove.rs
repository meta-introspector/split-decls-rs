// Generated macro for AutoRemove (enum)
macro_rules! DepcrateAutoRemove {
() => {
// Module: crate
// Provides: {"AutoRemove"}
// Dependencies: {}
# [doc = " A type expressing the ways we cleanup after ourselves to remove resources we created."] # [doc = " Note that cleanup has no effect if the tempfile is persisted."] # [derive (Debug , Clone , Ord , PartialOrd , Eq , PartialEq)] pub enum AutoRemove { # [doc = " Remove the temporary file after usage if it wasn't persisted."] Tempfile , # [doc = " Remove the temporary file as well the containing directories if they are empty until the given `directory`."] TempfileAndEmptyParentDirectoriesUntil { # [doc = " The directory which shall not be removed even if it is empty."] boundary_directory : PathBuf , } , }
};
}
