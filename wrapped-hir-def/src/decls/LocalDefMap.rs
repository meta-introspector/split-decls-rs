macro_rules! deps {
    () => {
        FxIndexMap!();
        DefMap!();
        CrateRootModuleId!();
    };
}

macro_rules! LocalDefMap {
    () => {
        deps!();
        # [doc = " Parts of the def map that are only needed when analyzing code in the same crate."] # [doc = ""] # [doc = " There are some data in the def map (e.g. extern prelude) that is only needed when analyzing"] # [doc = " things in the same crate (and maybe in the IDE layer), e.g. the extern prelude. If we put"] # [doc = " it in the DefMap dependant DefMaps will be invalidated when they change (e.g. when we add"] # [doc = " a dependency to the crate). Instead we split them out of the DefMap into a LocalDefMap struct."] # [doc = " `crate_local_def_map()` returns both, and `crate_def_map()` returns only the external-relevant"] # [doc = " DefMap."] # [derive (Debug , PartialEq , Eq , Default)] pub struct LocalDefMap { # [doc = " The extern prelude which contains all root modules of external crates that are in scope."] extern_prelude : FxIndexMap < Name , (CrateRootModuleId , Option < ExternCrateId >) > , }
    };
}

LocalDefMap!()