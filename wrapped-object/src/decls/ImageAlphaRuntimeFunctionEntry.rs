macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageAlphaRuntimeFunctionEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAlphaRuntimeFunctionEntry { pub begin_address : U32 < LE > , pub end_address : U32 < LE > , pub exception_handler : U32 < LE > , pub handler_data : U32 < LE > , pub prolog_end_address : U32 < LE > , }
    };
}

ImageAlphaRuntimeFunctionEntry!()