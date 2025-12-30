// Generated macro for Java_HelloWorld_counterDestroy (function)
macro_rules! DepcrateJava_HelloWorld_counterDestroy {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_counterDestroy"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "system" fn Java_HelloWorld_counterDestroy (_unowned_env : EnvUnowned , _class : JClass , counter_ptr : jlong ,) { let _boxed_counter = Box :: from_raw (counter_ptr as * mut Counter) ; }
};
}
