// Generated macro for autodetect_android_compiler (function)
macro_rules! Depcrateautodetect_android_compiler {
() => {
// Module: crate
// Provides: {"autodetect_android_compiler"}
// Dependencies: {}
fn autodetect_android_compiler (raw_target : & str , gnu : & str , clang : & str) -> PathBuf { let new_clang_key = match raw_target { "aarch64-linux-android" => Some ("aarch64") , "armv7-linux-androideabi" => Some ("armv7a") , "i686-linux-android" => Some ("i686") , "x86_64-linux-android" => Some ("x86_64") , _ => None , } ; let new_clang = new_clang_key . map (| key | { NEW_STANDALONE_ANDROID_COMPILERS . iter () . find (| x | x . starts_with (key)) }) . unwrap_or (None) ; if let Some (new_clang) = new_clang { if Command :: new (new_clang) . output () . is_ok () { return (* new_clang) . into () ; } } let target = raw_target . replace ("armv7neon" , "arm") . replace ("armv7" , "arm") . replace ("thumbv7neon" , "arm") . replace ("thumbv7" , "arm") ; let gnu_compiler = format ! ("{target}-{gnu}") ; let clang_compiler = format ! ("{target}-{clang}") ; let clang_compiler_cmd = format ! ("{target}-{clang}.cmd") ; if Command :: new (& gnu_compiler) . output () . is_ok () { gnu_compiler } else if cfg ! (windows) && Command :: new (& clang_compiler_cmd) . output () . is_ok () { clang_compiler_cmd } else { clang_compiler } . into () }
};
}
