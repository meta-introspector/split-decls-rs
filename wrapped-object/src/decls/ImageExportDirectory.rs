macro_rules! deps {
    () => {
        U32!();
        U16!();
    };
}

macro_rules! ImageExportDirectory {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageExportDirectory { pub characteristics : U32 < LE > , pub time_date_stamp : U32 < LE > , pub major_version : U16 < LE > , pub minor_version : U16 < LE > , pub name : U32 < LE > , pub base : U32 < LE > , pub number_of_functions : U32 < LE > , pub number_of_names : U32 < LE > , # [doc = " RVA from base of image"] pub address_of_functions : U32 < LE > , # [doc = " RVA from base of image"] pub address_of_names : U32 < LE > , # [doc = " RVA from base of image"] pub address_of_name_ordinals : U32 < LE > , }
    };
}

ImageExportDirectory!();