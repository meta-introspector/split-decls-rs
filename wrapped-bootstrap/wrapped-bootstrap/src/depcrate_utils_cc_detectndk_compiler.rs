// Generated macro for ndk_compiler (function)
macro_rules! Depcrate_utils_cc_detectndk_compiler {
() => {
// Module: crate::utils::cc_detect
// Provides: {"ndk_compiler"}
// Dependencies: {}
# [doc = " Constructs the path to the Android NDK compiler for the given target triple and language."] # [doc = ""] # [doc = " This helper function transform the target triple by converting certain architecture names"] # [doc = " (for example, translating \"arm\" to \"arm7a\"), appends the minimum API level (hardcoded as \"21\""] # [doc = " for NDK r26d), and then constructs the full path based on the provided NDK directory and host"] # [doc = " platform."] pub (crate) fn ndk_compiler (compiler : Language , triple : & str , ndk : & Path) -> PathBuf { let mut triple_iter = triple . split ('-') ; let triple_translated = if let Some (arch) = triple_iter . next () { let arch_new = match arch { "arm" | "armv7" | "armv7neon" | "thumbv7" | "thumbv7neon" => "armv7a" , other => other , } ; std :: iter :: once (arch_new) . chain (triple_iter) . collect :: < Vec < & str > > () . join ("-") } else { triple . to_string () } ; let api_level = "21" ; let compiler = format ! ("{}{}-{}" , triple_translated , api_level , compiler . clang ()) ; let host_tag = if cfg ! (target_os = "macos") { "darwin-x86_64" } else if cfg ! (target_os = "windows") { "windows-x86_64" } else { "linux-x86_64" } ; ndk . join ("toolchains") . join ("llvm") . join ("prebuilt") . join (host_tag) . join ("bin") . join (compiler) }
};
}
