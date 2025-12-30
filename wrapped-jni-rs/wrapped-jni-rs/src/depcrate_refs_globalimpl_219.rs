// Generated macro for impl_219 (impl)
macro_rules! Depcrate_refs_globalimpl_219 {
() => {
// Module: crate::refs::global
// Provides: {"impl_219"}
// Dependencies: {}
impl < T > Drop for Global < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync + 'static , { fn drop (& mut self) { let obj = std :: mem :: take (& mut self . obj) ; if ! obj . is_null () { let vm = JavaVM :: singleton () . expect ("JavaVM singleton uninitialized") ; let res = vm . attach_current_thread_for_scope (| env | -> Result < () > { if env . owns_attachment () { warn ! ("A JNI global reference was dropped on a thread that is not attached. This will cause a performance problem if it happens frequently. For more information, see the documentation for `jni::objects::Global`.") ; } unsafe { jni_call_unchecked ! (env , v1_1 , DeleteGlobalRef , obj . as_raw ()) ; } Ok (()) } ,) ; if let Err (err) = res { debug ! ("error dropping global ref: {:#?}" , err) ; } } } }
};
}
