macro_rules! deps {
    () => {
        U32!();
        U16!();
        U64!();
    };
}

macro_rules! NonPagedDebugInfo {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct NonPagedDebugInfo { pub signature : U16 < LE > , pub flags : U16 < LE > , pub size : U32 < LE > , pub machine : U16 < LE > , pub characteristics : U16 < LE > , pub time_date_stamp : U32 < LE > , pub check_sum : U32 < LE > , pub size_of_image : U32 < LE > , pub image_base : U64 < LE > , }
    };
}

NonPagedDebugInfo!()