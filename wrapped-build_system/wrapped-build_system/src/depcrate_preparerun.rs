// Generated macro for run (function)
macro_rules! Depcrate_preparerun {
() => {
// Module: crate::prepare
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { let args = match PrepareArg :: new () ? { Some (a) => a , None => return Ok (()) , } ; let sysroot_path = get_sysroot_dir () ; prepare_libcore (& sysroot_path , args . libgccjit12_patches , args . cross_compile , args . sysroot_source ,) ? ; if ! args . only_libcore { cargo_install ("hyperfine") ? ; let to_clone = & [("https://github.com/rust-random/rand.git" , "1f4507a8e1cf8050e4ceef95eeda8f64645b6719" , None ,) , ("https://github.com/rust-lang/regex.git" , "341f207c1071f7290e3f228c710817c280c8dca1" , None ,) , ("https://github.com/ebobby/simple-raytracer" , "804a7a21b9e673a482797aa289a18ed480e4d813" , Some (build_raytracer) ,) ,] ; for (repo_url , checkout_commit , cb) in to_clone { clone_and_setup (repo_url , checkout_commit , * cb) ? ; } prepare_rand () ? ; } println ! ("Successfully ran `prepare`") ; Ok (()) }
};
}
