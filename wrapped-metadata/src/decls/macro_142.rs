macro_rules! deps {
    () => {
        ModuleRef!();
        Module!();
        AssemblyRef!();
        TypeRef!();
    };
}

macro_rules! macro_142 {
    () => {
        deps!();
        code ! { ResolutionScope (2) (Module , 0) (ModuleRef , 1) (AssemblyRef , 2) (TypeRef , 3) }
    };
}

macro_142!()