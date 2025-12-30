// Generated macro for discover (function)
macro_rules! Depcrate_organizediscover {
() => {
// Module: crate::organize
// Provides: {"discover"}
// Dependencies: {}
# [doc = " Find all working directories in the given `source_dir` and print them to `out` while providing `progress`."] pub fn discover < P : NestedProgress > (source_dir : impl AsRef < Path > , mut out : impl std :: io :: Write , mut progress : P , debug : bool , threads : Option < usize > ,) -> anyhow :: Result < () > { for (git_workdir , _kind) in find_git_repository_workdirs (source_dir , progress . add_child ("Searching repositories") , debug , threads) { writeln ! (& mut out , "{}" , git_workdir . display ()) ? ; } Ok (()) }
};
}
