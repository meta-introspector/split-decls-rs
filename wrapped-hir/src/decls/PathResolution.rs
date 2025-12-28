macro_rules! deps {
    () => {
        TypeParam!();
        DeriveHelper!();
        Impl!();
        ConstParam!();
        ModuleDef!();
        ToolModule!();
        Local!();
        BuiltinAttr!();
    };
}

macro_rules! PathResolution {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum PathResolution { # [doc = " An item"] Def (ModuleDef) , # [doc = " A local binding (only value namespace)"] Local (Local) , # [doc = " A type parameter"] TypeParam (TypeParam) , # [doc = " A const parameter"] ConstParam (ConstParam) , SelfType (Impl) , BuiltinAttr (BuiltinAttr) , ToolModule (ToolModule) , DeriveHelper (DeriveHelper) , }
    };
}

PathResolution!()