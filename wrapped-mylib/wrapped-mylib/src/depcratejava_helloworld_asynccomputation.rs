// Generated macro for Java_HelloWorld_asyncComputation (function)
macro_rules! DepcrateJava_HelloWorld_asyncComputation {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_asyncComputation"}
// Dependencies: {}
# [no_mangle] pub extern "system" fn Java_HelloWorld_asyncComputation (mut unowned_env : EnvUnowned , _class : JClass , callback : JObject ,) { unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let jvm = env . get_java_vm () ; let callback = env . new_global_ref (callback) . unwrap () ; let (tx , rx) = mpsc :: channel () ; let _ = thread :: spawn (move | | { tx . send (()) . unwrap () ; jvm . attach_current_thread (| env | -> jni :: errors :: Result < () > { for i in 0 .. 11 { let progress = (i * 10) as jint ; env . call_method (& callback , c"asyncCallback" , c"(I)V" , & [progress . into ()]) . unwrap () ; thread :: sleep (Duration :: from_millis (100)) ; } Ok (()) }) . unwrap () ; }) ; rx . recv () . unwrap () ; Ok (()) }) . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
