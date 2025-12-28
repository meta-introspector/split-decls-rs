macro_rules! deps {
    () => {
        FxIndexMap!();
        ItemScope!();
        DefMap!();
        LocalModuleId!();
        ModuleOrigin!();
        Visibility!();
    };
}

macro_rules! ModuleData {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct ModuleData { # [doc = " Where does this module come from?"] pub origin : ModuleOrigin , # [doc = " Declared visibility of this module."] pub visibility : Visibility , # [doc = " Parent module in the same `DefMap`."] # [doc = ""] # [doc = " [`None`] for block modules because they are always its `DefMap`'s root."] pub parent : Option < LocalModuleId > , pub children : FxIndexMap < Name , LocalModuleId > , pub scope : ItemScope , }
    };
}

ModuleData!()