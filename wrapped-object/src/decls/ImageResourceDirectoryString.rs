macro_rules! deps {
    () => {
        U16!();
    };
}

macro_rules! ImageResourceDirectoryString {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDirectoryString { pub length : U16 < LE > , }
    };
}

ImageResourceDirectoryString!();