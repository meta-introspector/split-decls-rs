macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageBaseRelocation {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageBaseRelocation { pub virtual_address : U32 < LE > , pub size_of_block : U32 < LE > , }
    };
}

ImageBaseRelocation!()