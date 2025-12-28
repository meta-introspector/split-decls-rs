macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageDynamicRelocationTable {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDynamicRelocationTable { pub version : U32 < LE > , pub size : U32 < LE > , }
    };
}

ImageDynamicRelocationTable!();