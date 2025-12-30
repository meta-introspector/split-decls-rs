// Generated macro for Java_HelloWorld_hello (function)
macro_rules! DepcrateJava_HelloWorld_hello {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_hello"}
// Dependencies: {}
# [no_mangle] pub extern "system" fn Java_HelloWorld_hello < 'local > (mut unowned_env : EnvUnowned < 'local > , _class : JClass < 'local > , input : JString < 'local > ,) -> JString < 'local > { let outcome = unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let input : String = input . to_string () ; env . new_string (JNIString :: from (format ! ("Hello, {}!" , input))) }) ; outcome . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
