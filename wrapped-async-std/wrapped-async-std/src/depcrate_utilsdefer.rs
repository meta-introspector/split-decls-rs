// Generated macro for defer (macro)
macro_rules! Depcrate_utilsdefer {
() => {
// Module: crate::utils
// Provides: {"defer"}
// Dependencies: {}
# [doc = " Defers evaluation of a block of code until the end of the scope."] # [cfg (feature = "default")] # [doc (hidden)] macro_rules ! defer { ($ ($ body : tt) *) => { let _guard = { pub struct Guard < F : FnOnce () > (Option < F >) ; impl < F : FnOnce () > Drop for Guard < F > { fn drop (& mut self) { (self . 0) . take () . map (| f | f ()) ; } } Guard (Some (|| { let _ = { $ ($ body) * } ; })) } ; } ; }
};
}
