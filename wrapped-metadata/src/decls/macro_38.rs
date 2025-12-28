macro_rules! deps {
    () => {
        ModuleRef!();
        AssemblyRef!();
        Module!();
        TypeRef!();
    };
}

macro_rules! macro_38 {
    () => {
        deps!();
        code ! { ResolutionScope (2) (Module , 0) (ModuleRef , 1) (AssemblyRef , 2) (TypeRef , 3) }
    };
}

macro_38!();