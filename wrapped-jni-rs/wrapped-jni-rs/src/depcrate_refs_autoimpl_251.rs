// Generated macro for impl_251 (impl)
macro_rules! Depcrate_refs_autoimpl_251 {
() => {
// Module: crate::refs::auto
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'local , T > Drop for Auto < 'local , T > where T : Into < JObject < 'local > > , { fn drop (& mut self) { let obj = unsafe { ManuallyDrop :: take (& mut self . obj) } ; let obj : JObject = obj . into () ; if ! obj . is_null () { let Ok (vm) = JavaVM :: singleton () else { log :: error ! ("Failed to drop Auto: No JavaVM initialized") ; return ; } ; vm . with_top_local_frame (| env | -> errors :: Result < () > { env . delete_local_ref (obj) ; Ok (()) }) . expect ("Infallible") ; } } }
};
}
