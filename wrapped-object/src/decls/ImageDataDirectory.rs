macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageDataDirectory {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDataDirectory { pub virtual_address : U32 < LE > , pub size : U32 < LE > , }
    };
}

ImageDataDirectory!()