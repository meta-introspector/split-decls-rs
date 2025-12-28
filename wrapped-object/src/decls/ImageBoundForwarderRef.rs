macro_rules! deps {
    () => {
        U16!();
        U32!();
    };
}

macro_rules! ImageBoundForwarderRef {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageBoundForwarderRef { pub time_date_stamp : U32 < LE > , pub offset_module_name : U16 < LE > , pub reserved : U16 < LE > , }
    };
}

ImageBoundForwarderRef!()