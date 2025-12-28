macro_rules! deps {
    () => {
        VsInstance!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl VsInstance { pub fn installation_name (& self) -> Option < Cow < '_ , str > > { match self { VsInstance :: Com (s) => s . installation_name () . ok () . and_then (| s | s . into_string () . ok ()) . map (Cow :: from) , VsInstance :: Vswhere (v) => v . map . get ("installationName") . map (Cow :: from) , } } pub fn installation_path (& self) -> Option < PathBuf > { match self { VsInstance :: Com (s) => s . installation_path () . ok () . map (PathBuf :: from) , VsInstance :: Vswhere (v) => v . map . get ("installationPath") . map (PathBuf :: from) , } } pub fn installation_version (& self) -> Option < Cow < '_ , str > > { match self { VsInstance :: Com (s) => s . installation_version () . ok () . and_then (| s | s . into_string () . ok ()) . map (Cow :: from) , VsInstance :: Vswhere (v) => v . map . get ("installationVersion") . map (Cow :: from) , } } }
    };
}

impl_156!();