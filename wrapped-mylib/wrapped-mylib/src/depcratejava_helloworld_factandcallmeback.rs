// Generated macro for Java_HelloWorld_factAndCallMeBack (function)
macro_rules! DepcrateJava_HelloWorld_factAndCallMeBack {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_factAndCallMeBack"}
// Dependencies: {}
# [no_mangle] pub extern "system" fn Java_HelloWorld_factAndCallMeBack (mut unowned_env : EnvUnowned , _class : JClass , n : jint , callback : JObject ,) { unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let i = n as i32 ; let res : jint = (2 .. i + 1) . product () ; env . call_method (callback , c"factCallback" , c"(I)V" , & [res . into ()]) . unwrap () ; Ok (()) }) . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
