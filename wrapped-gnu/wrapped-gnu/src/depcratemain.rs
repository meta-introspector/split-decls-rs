// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { const ALL_PLATFORMS : [& str ; 5] = ["x86_64_gnu" , "i686_gnu" , "x86_64_gnullvm" , "aarch64_gnullvm" , "i686_gnullvm" ,] ; let mut platforms = HashSet :: new () ; for platform in std :: env :: args () . skip (1) { if ALL_PLATFORMS . contains (& & * platform) { platforms . insert (platform) ; } else if platform == "all" { platforms . extend (ALL_PLATFORMS . iter () . map (| s | s . to_string ())) ; } else { eprintln ! ("Unknown platform: {platform}") ; return ; } } if platforms . is_empty () { eprintln ! ("Please specify at least one platform or use 'all' argument") ; return ; } ; for platform in platforms { let tools = if platform . ends_with ("_gnu") { & ["dlltool" , "ar" , "objcopy"] [..] } else { & ["llvm-dlltool" , "llvm-ar"] [..] } ; for tool in tools { if Command :: new (tool) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () . is_err () { eprintln ! ("Could not find {tool}. Is it in your $PATH?") ; std :: process :: exit (1) ; } } build_platform (& platform , tools [0] , tools [1]) ; } }
};
}
