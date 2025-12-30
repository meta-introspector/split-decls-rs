// Generated macro for impl_274 (impl)
macro_rules! Depcrate_objects_jobjectimpl_274 {
() => {
// Module: crate::objects::jobject
// Provides: {"impl_274"}
// Dependencies: {}
impl JObject < '_ > { # [doc = " Creates a [`JObject`] that wraps the given `raw` [`jobject`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `raw` must be a valid raw JNI local reference (or `null`)."] # [doc = " - There must not be any other owning [`Reference`] wrapper for the same reference."] # [doc = " - The local reference must belong to the current thread and not outlive the"] # [doc = "   JNI stack frame associated with the [Env] `'local` lifetime."] pub unsafe fn from_raw < 'local > (_env : & Env < 'local > , raw : jobject) -> JObject < 'local > { JObject :: kind_from_raw (raw) } # [doc = " Creates a new null reference."] # [doc = ""] # [doc = " Null references are always valid and do not belong to a local reference frame. Therefore,"] # [doc = " the returned `JObject` always has the `'static` lifetime."] pub const fn null () -> JObject < 'static > { JObject { internal : std :: ptr :: null_mut () , lifetime : PhantomData , } } # [doc = " Returns the raw JNI pointer."] pub const fn as_raw (& self) -> jobject { self . internal } # [doc = " Unwrap to the internal jni type."] pub const fn into_raw (self) -> jobject { self . internal } }
};
}
