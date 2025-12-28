macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl From < PathResolution > for Definition { fn from (path_resolution : PathResolution) -> Self { match path_resolution { PathResolution :: Def (def) => def . into () , PathResolution :: Local (local) => Definition :: Local (local) , PathResolution :: TypeParam (par) => Definition :: GenericParam (par . into ()) , PathResolution :: ConstParam (par) => Definition :: GenericParam (par . into ()) , PathResolution :: SelfType (impl_def) => Definition :: SelfType (impl_def) , PathResolution :: BuiltinAttr (attr) => Definition :: BuiltinAttr (attr) , PathResolution :: ToolModule (tool) => Definition :: ToolModule (tool) , PathResolution :: DeriveHelper (helper) => Definition :: DeriveHelper (helper) , } } }
    };
}

impl_41!();