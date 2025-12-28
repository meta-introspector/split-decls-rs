macro_rules! deps {
    () => {
        CrateBuilderId!();
        CrateDisplayName!();
    };
}

macro_rules! CyclicDependenciesError {
    () => {
        deps!();
        # [derive (Debug)] pub struct CyclicDependenciesError { path : Vec < (CrateBuilderId , Option < CrateDisplayName >) > , }
    };
}

CyclicDependenciesError!();