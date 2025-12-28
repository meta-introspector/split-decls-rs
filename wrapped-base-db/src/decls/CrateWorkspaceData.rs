macro_rules! deps {
    () => {
        TargetData!();
        Crate!();
        TargetLoadError!();
    };
}

macro_rules! CrateWorkspaceData {
    () => {
        deps!();
        # [doc = " Crate related data shared by the whole workspace."] # [derive (Debug , PartialEq , Eq , Hash , Clone)] pub struct CrateWorkspaceData { pub target : Result < target :: TargetData , target :: TargetLoadError > , # [doc = " Toolchain version used to compile the crate."] pub toolchain : Option < Version > , }
    };
}

CrateWorkspaceData!();