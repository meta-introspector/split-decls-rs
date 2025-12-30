// Generated macro for Java_HelloWorld_counterNew (function)
macro_rules! DepcrateJava_HelloWorld_counterNew {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_counterNew"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "system" fn Java_HelloWorld_counterNew (mut unowned_env : EnvUnowned , _class : JClass , callback : JObject ,) -> jlong { unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let global_ref = env . new_global_ref (callback) . unwrap () ; let counter = Counter :: new (global_ref) ; Ok (Box :: into_raw (Box :: new (counter)) as jlong) }) . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
