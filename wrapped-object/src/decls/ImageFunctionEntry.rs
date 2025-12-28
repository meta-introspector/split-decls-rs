macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageFunctionEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageFunctionEntry { pub starting_address : U32 < LE > , pub ending_address : U32 < LE > , pub end_of_prologue : U32 < LE > , }
    };
}

ImageFunctionEntry!()