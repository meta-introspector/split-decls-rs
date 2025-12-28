macro_rules! deps {
    () => {
        CrateDisplayName!();
        CrateBuilderId!();
    };
}

macro_rules! CyclicDependenciesError {
    () => {
        deps!();
        # [derive (Debug)] pub struct CyclicDependenciesError { path : Vec < (CrateBuilderId , Option < CrateDisplayName >) > , }
    };
}

CyclicDependenciesError!()