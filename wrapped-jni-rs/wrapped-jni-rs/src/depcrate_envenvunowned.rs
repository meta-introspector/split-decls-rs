// Generated macro for EnvUnowned (struct)
macro_rules! Depcrate_envEnvUnowned {
() => {
// Module: crate::env
// Provides: {"EnvUnowned"}
// Dependencies: {}
# [doc = " Represents an external (unowned) JNI stack frame and thread attachment that"] # [doc = " was passed to a native method call."] # [doc = ""] # [doc = " This is an FFI safe wrapper around a [`crate::sys::JNIEnv`] pointer that has"] # [doc = " been passed as the first argument to a native method call, and represents"] # [doc = " an implicit JNI thread attachment."] # [doc = ""] # [doc = " For example, you can use it with a native method implementation like this:"] # [doc = " ```rust,no_run"] # [doc = " # use jni::objects::{JObject, JString};"] # [doc = " # use jni::errors::ThrowRuntimeExAndDefault;"] # [doc = " #[no_mangle]"] # [doc = " pub extern \"system\" fn Java_com_example_MyClass_myNativeMethod<'caller>("] # [doc = "     mut unowned_env: jni::EnvUnowned<'caller>,"] # [doc = "     _this: JObject<'caller>,"] # [doc = "     arg: JString<'caller>,"] # [doc = " ) -> JObject<'caller> {"] # [doc = "     unowned_env.with_env(|env| -> jni::errors::Result<_> {"] # [doc = "         // Use `env` to call Java methods or access fields."] # [doc = "         Ok(JObject::null())"] # [doc = "     }).resolve::<ThrowRuntimeExAndDefault>()"] # [doc = " }"] # [doc = " ```"] # [repr (transparent)] # [derive (Debug)] pub struct EnvUnowned < 'local > { ptr : * mut jni_sys :: JNIEnv , _lifetime : std :: marker :: PhantomData < & 'local () > , }
};
}
