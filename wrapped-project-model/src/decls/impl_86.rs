macro_rules! deps {
    () => {
        RustLibSrcWorkspace!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl fmt :: Display for RustLibSrcWorkspace { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RustLibSrcWorkspace :: Workspace { ws , .. } => { write ! (f , "workspace {}" , ws . workspace_root ()) } RustLibSrcWorkspace :: Json (json) => write ! (f , "json {}" , json . manifest_or_root ()) , RustLibSrcWorkspace :: Stitched (stitched) => { write ! (f , "stitched with {} crates" , stitched . crates . len ()) } RustLibSrcWorkspace :: Empty => write ! (f , "empty") , } } }
    };
}

impl_86!();