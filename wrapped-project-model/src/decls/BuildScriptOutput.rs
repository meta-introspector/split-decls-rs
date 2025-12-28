macro_rules! deps {
    () => {
        ProcMacroDylibPath!();
    };
}

macro_rules! BuildScriptOutput {
    () => {
        deps!();
        # [doc = " Output of the build script and proc-macro building step for a concrete package."] # [derive (Debug , Clone , Default , PartialEq , Eq)] pub (crate) struct BuildScriptOutput { # [doc = " List of config flags defined by this package's build script."] pub (crate) cfgs : Vec < CfgAtom > , # [doc = " List of cargo-related environment variables with their value."] # [doc = ""] # [doc = " If the package has a build script which defines environment variables,"] # [doc = " they can also be found here."] pub (crate) envs : Env , # [doc = " Directory where a build script might place its output."] pub (crate) out_dir : Option < AbsPathBuf > , # [doc = " Path to the proc-macro library file if this package exposes proc-macros."] pub (crate) proc_macro_dylib_path : ProcMacroDylibPath , }
    };
}

BuildScriptOutput!();