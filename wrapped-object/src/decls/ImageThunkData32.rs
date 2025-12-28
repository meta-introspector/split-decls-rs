macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageThunkData32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageThunkData32 (pub U32 < LE >) ;
    };
}

ImageThunkData32!()