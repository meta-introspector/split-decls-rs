// Generated macro for JNIEnv (type)
macro_rules! DepcrateJNIEnv {
() => {
// Module: crate
// Provides: {"JNIEnv"}
// Dependencies: {}
# [deprecated (since = "0.22.0" , note = r#"Since 0.22, `JNIEnv` (renamed `Env`) is not an FFI safe pointer wrapper any more.

To remain safe by default, `jni::JNIEnv` is now an alias for `EnvUnowned` (which is FFI safe).

Use `EnvUnowned` to capture a raw `jni_sys::JNIEnv` pointer in native methods, like:
`pub extern "system" fn Java_HelloWorld_hello<'frame>(unowned_env: EnvUnowned<'frame>, ...)`
Then use `unowned_env.with_env()` to upgrade it to an `Env` reference.

Most of the time you should temporarily acquire a `Env` reference using:
- `JavaVM::attach_current_thread` (preferred) or `JavaVM::attach_current_thread_for_scope`
- `JavaVM::with_env` (if certain that the thread is already attached)
- `UnownedEnv::with_env()` in native methods

When migrating to 0.22, if you have a mixture of FFI/non-FFI usage of JNIEnv then err on the side
of renaming `JNIEnv` to `EnvUnowned` because that's safe for FFI and for non-FFI usage there will be
clear compiler errors if trying to access the real `Env` API through the `EnvUnowned` type.
"#)] # [doc = " An FFI safe alias for `EnvUnowned` for (safer) compatibility with"] # [doc = " existing code."] # [doc = ""] # [doc = " Since 0.22 ([#570](https://github.com/jni-rs/jni-rs/pull/570)) the"] # [doc = " `JNIEnv` type was renamed to [Env], which is no longer an FFI safe"] # [doc = " pointer wrapper."] # [doc = ""] # [doc = " FFI usage of `JNIEnv`, within native method arguments, should be"] # [doc = " migrated to [EnvUnowned], followed by [`EnvUnowned::with_env`]."] # [doc = ""] # [doc = " To help make this clear and sign post how to safely migrate to the [Env]"] # [doc = " and [EnvUnowned] types, we export this deprecated alias with a warning."] pub type JNIEnv < 'frame > = EnvUnowned < 'frame > ;
};
}
