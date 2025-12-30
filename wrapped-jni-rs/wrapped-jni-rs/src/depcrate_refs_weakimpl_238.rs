// Generated macro for impl_238 (impl)
macro_rules! Depcrate_refs_weakimpl_238 {
() => {
// Module: crate::refs::weak
// Provides: {"impl_238"}
// Dependencies: {}
impl < T > Drop for Weak < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { fn drop (& mut self) { let obj = std :: mem :: take (& mut self . obj) ; if ! obj . is_null () { let vm = JavaVM :: singleton () . expect ("JavaVM singleton uninitialized") ; let res = vm . attach_current_thread_for_scope (| env | -> Result < () > { if env . owns_attachment () { warn ! ("Dropping a Weak in a detached thread. Fix your code if this message appears frequently (see the Weak docs).") ; } unsafe { jni_call_unchecked ! (env , v1_2 , DeleteWeakGlobalRef , obj . as_raw ()) ; } Ok (()) }) ; if let Err (err) = res { debug ! ("error dropping weak ref: {:#?}" , err) ; } } } }
};
}
