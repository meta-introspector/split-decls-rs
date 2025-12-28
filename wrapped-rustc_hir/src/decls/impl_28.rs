macro_rules! deps {
    () => {
        NativeLibKind!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl NativeLibKind { pub fn has_modifiers (& self) -> bool { match self { NativeLibKind :: Static { bundle , whole_archive } => { bundle . is_some () || whole_archive . is_some () } NativeLibKind :: Dylib { as_needed } | NativeLibKind :: Framework { as_needed } => { as_needed . is_some () } NativeLibKind :: RawDylib | NativeLibKind :: Unspecified | NativeLibKind :: LinkArg | NativeLibKind :: WasmImportModule => false , } } pub fn is_statically_included (& self) -> bool { matches ! (self , NativeLibKind :: Static { .. }) } pub fn is_dllimport (& self) -> bool { matches ! (self , NativeLibKind :: Dylib { .. } | NativeLibKind :: RawDylib | NativeLibKind :: Unspecified) } }
    };
}

impl_28!()