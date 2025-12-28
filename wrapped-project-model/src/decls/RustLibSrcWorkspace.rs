macro_rules! deps {
    () => {
        CargoWorkspace!();
        ProjectJson!();
    };
}

macro_rules! RustLibSrcWorkspace {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum RustLibSrcWorkspace { Workspace { ws : CargoWorkspace , metadata_err : Option < String > } , Json (ProjectJson) , Stitched (stitched :: Stitched) , Empty , }
    };
}

RustLibSrcWorkspace!()