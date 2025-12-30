// Generated macro for impl_40 (impl)
macro_rules! Depcrate_fseventimpl_40 {
() => {
// Module: crate::fsevent
// Provides: {"impl_40"}
// Dependencies: {}
impl PathsMut for FsEventPathsMut < '_ > { fn add (& mut self , path : & Path , recursive_mode : RecursiveMode) -> Result < () > { self . 0 . append_path (path , recursive_mode) } fn remove (& mut self , path : & Path) -> Result < () > { self . 0 . remove_path (path) } fn commit (self : Box < Self >) -> Result < () > { let _ = self . 0 . run () ; Ok (()) } }
};
}
