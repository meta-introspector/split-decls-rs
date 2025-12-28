macro_rules! deps {
    () => {
        U32!();
        U16!();
    };
}

macro_rules! ImageResourceDirectory {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDirectory { pub characteristics : U32 < LE > , pub time_date_stamp : U32 < LE > , pub major_version : U16 < LE > , pub minor_version : U16 < LE > , pub number_of_named_entries : U16 < LE > , pub number_of_id_entries : U16 < LE > , }
    };
}

ImageResourceDirectory!();