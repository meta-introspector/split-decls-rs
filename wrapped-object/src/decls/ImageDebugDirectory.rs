macro_rules! deps {
    () => {
        U16!();
        U32!();
    };
}

macro_rules! ImageDebugDirectory {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDebugDirectory { pub characteristics : U32 < LE > , pub time_date_stamp : U32 < LE > , pub major_version : U16 < LE > , pub minor_version : U16 < LE > , pub typ : U32 < LE > , pub size_of_data : U32 < LE > , pub address_of_raw_data : U32 < LE > , pub pointer_to_raw_data : U32 < LE > , }
    };
}

ImageDebugDirectory!();