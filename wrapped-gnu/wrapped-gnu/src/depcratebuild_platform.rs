// Generated macro for build_platform (function)
macro_rules! Depcratebuild_platform {
() => {
// Module: crate
// Provides: {"build_platform"}
// Dependencies: {}
fn build_platform (platform : & str , dlltool : & str , ar : & str) { println ! ("Platform: {platform}") ; let libraries = libraries () ; let output = std :: path :: PathBuf :: from (format ! ("crates/targets/{platform}/lib")) ; std :: fs :: create_dir_all (& output) . unwrap () ; std :: fs :: create_dir_all (& output) . unwrap () ; for (library , functions) in & libraries { build_library (& output , dlltool , library , functions , platform) ; } build_mri (& output , ar , & libraries) ; for library in libraries . keys () { std :: fs :: remove_file (output . join (format ! ("lib{library}.a"))) . unwrap () ; } }
};
}
