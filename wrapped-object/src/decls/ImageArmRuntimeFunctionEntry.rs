macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageArmRuntimeFunctionEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageArmRuntimeFunctionEntry { pub begin_address : U32 < LE > , pub unwind_data : U32 < LE > , }
    };
}

ImageArmRuntimeFunctionEntry!()