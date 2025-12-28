macro_rules! deps {
    () => {
        Package!();
        BuildScriptOutput!();
    };
}

macro_rules! WorkspaceBuildScripts {
    () => {
        deps!();
        # [doc = " Output of the build script and proc-macro building steps for a workspace."] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct WorkspaceBuildScripts { outputs : ArenaMap < Package , BuildScriptOutput > , error : Option < String > , }
    };
}

WorkspaceBuildScripts!()