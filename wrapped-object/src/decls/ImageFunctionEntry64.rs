macro_rules! deps {
    () => {
        U64!();
    };
}

macro_rules! ImageFunctionEntry64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageFunctionEntry64 { pub starting_address : U64 < LE > , pub ending_address : U64 < LE > , pub end_of_prologue_or_unwind_info_address : U64 < LE > , }
    };
}

ImageFunctionEntry64!();