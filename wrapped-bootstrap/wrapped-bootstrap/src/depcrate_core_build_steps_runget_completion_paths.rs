// Generated macro for get_completion_paths (function)
macro_rules! Depcrate_core_build_steps_runget_completion_paths {
() => {
// Module: crate::core::build_steps::run
// Provides: {"get_completion_paths"}
// Dependencies: {}
# [doc = " Return tuples of (shell, file containing completions)."] pub fn get_completion_paths (builder : & Builder < '_ >) -> Vec < (& 'static dyn Generator , PathBuf) > { vec ! [(& shells :: Bash as &'static dyn Generator , builder . src . join ("src/etc/completions/x.py.sh")) , (& shells :: Zsh , builder . src . join ("src/etc/completions/x.py.zsh")) , (& shells :: Fish , builder . src . join ("src/etc/completions/x.py.fish")) , (& shells :: PowerShell , builder . src . join ("src/etc/completions/x.py.ps1")) , (& shells :: Bash , builder . src . join ("src/etc/completions/x.sh")) , (& shells :: Zsh , builder . src . join ("src/etc/completions/x.zsh")) , (& shells :: Fish , builder . src . join ("src/etc/completions/x.fish")) , (& shells :: PowerShell , builder . src . join ("src/etc/completions/x.ps1")) ,] }
};
}
