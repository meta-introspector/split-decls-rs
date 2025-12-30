// Generated macro for Java_HelloWorld_helloByte (function)
macro_rules! DepcrateJava_HelloWorld_helloByte {
() => {
// Module: crate
// Provides: {"Java_HelloWorld_helloByte"}
// Dependencies: {}
# [no_mangle] pub extern "system" fn Java_HelloWorld_helloByte < 'local > (mut unowned_env : EnvUnowned < 'local > , _class : JClass , input : JByteArray ,) -> JByteArray < 'local > { unowned_env . with_env (| env | -> jni :: errors :: Result < _ > { let _input = env . convert_byte_array (& input) . unwrap () ; let buf = [1 ; 2000] ; let output = env . byte_array_from_slice (& buf) . unwrap () ; Ok (output) }) . resolve :: < jni :: errors :: ThrowRuntimeExAndDefault > () }
};
}
