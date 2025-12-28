macro_rules! deps {
    () => {
        U16!();
    };
}

macro_rules! ImageResourceDirStringU {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDirStringU { pub length : U16 < LE > , }
    };
}

ImageResourceDirStringU!();