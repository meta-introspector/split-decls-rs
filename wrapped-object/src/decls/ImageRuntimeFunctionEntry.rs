macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageRuntimeFunctionEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRuntimeFunctionEntry { pub begin_address : U32 < LE > , pub end_address : U32 < LE > , pub unwind_info_address_or_data : U32 < LE > , }
    };
}

ImageRuntimeFunctionEntry!();