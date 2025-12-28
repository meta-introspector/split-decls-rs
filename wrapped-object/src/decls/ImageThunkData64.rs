macro_rules! deps {
    () => {
        U64!();
    };
}

macro_rules! ImageThunkData64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageThunkData64 (pub U64 < LE >) ;
    };
}

ImageThunkData64!();