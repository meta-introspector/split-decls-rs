// Generated macro for JPrimitiveArray (struct)
macro_rules! Depcrate_objects_jprimitive_arrayJPrimitiveArray {
() => {
// Module: crate::objects::jprimitive_array
// Provides: {"JPrimitiveArray"}
// Dependencies: {}
# [doc = " A primitive array wrapper that is tied to a JNI local reference frame."] # [doc = ""] # [doc = " This is a wrapper type for a local JNI reference that's used to"] # [doc = " differentiate primitive array types like `boolean[]` or `int[]`."] # [doc = ""] # [doc = " For convenience it's recommended to use one of the type aliases like:"] # [doc = " - [JBooleanArray]"] # [doc = " - [JByteArray]"] # [doc = " - [JCharArray]"] # [doc = " - [JShortArray]"] # [doc = " - [JIntArray]"] # [doc = " - [JLongArray]"] # [doc = " - [JFloatArray]"] # [doc = " - [JDoubleArray]"] # [doc = ""] # [doc = " See [JObjectArray] for non-primitive object arrays."] # [doc = ""] # [doc = " See the [`JObject`] documentation for more information about reference"] # [doc = " wrappers, how to cast them, and local reference frame lifetimes."] # [doc = ""] # [repr (transparent)] # [derive (Debug)] pub struct JPrimitiveArray < 'local , T : TypeArray > { obj : JObject < 'local > , _marker : PhantomData < T > , }
};
}
