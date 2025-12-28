macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageArm64RuntimeFunctionEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageArm64RuntimeFunctionEntry { pub begin_address : U32 < LE > , pub unwind_data : U32 < LE > , }
    };
}

ImageArm64RuntimeFunctionEntry!()