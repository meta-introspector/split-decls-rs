// Generated macro for JObjectArray (struct)
macro_rules! Depcrate_objects_jobject_arrayJObjectArray {
() => {
// Module: crate::objects::jobject_array
// Provides: {"JObjectArray"}
// Dependencies: {}
# [doc = " A `java.lang.Object[]` wrapper that is tied to a JNI local reference frame."] # [doc = ""] # [doc = " See the [`JObject`] documentation for more information about reference"] # [doc = " wrappers, how to cast them, and local reference frame lifetimes."] # [doc = ""] # [repr (transparent)] # [derive (Debug , Default)] pub struct JObjectArray < 'local , E : Reference + 'local = JObject < 'local > > { array : JObject < 'local > , _marker : std :: marker :: PhantomData < E > , }
};
}
