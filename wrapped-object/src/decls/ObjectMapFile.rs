macro_rules! deps {
    () => {
        ObjectMap!();
    };
}

macro_rules! ObjectMapFile {
    () => {
        deps!();
        # [doc = " An object file name in an [`ObjectMap`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ObjectMapFile < 'data > { path : & 'data [u8] , member : Option < & 'data [u8] > , }
    };
}

ObjectMapFile!();