// Generated macro for Java_HelloWorld_counterIncrement (function)
macro_rules! DepcrateJava_HelloWorld_counterIncrement {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_counterIncrement"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "system" fn Java_HelloWorld_counterIncrement (mut unowned_env : EnvUnowned , _class : JClass , counter_ptr : jlong ,) { unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let counter = & mut * (counter_ptr as * mut Counter) ; counter . increment (env) ; Ok (()) }) . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
