macro_rules! deps {
    () => {
        U16!();
        U32!();
    };
}

macro_rules! ImageBoundImportDescriptor {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageBoundImportDescriptor { pub time_date_stamp : U32 < LE > , pub offset_module_name : U16 < LE > , pub number_of_module_forwarder_refs : U16 < LE > , }
    };
}

ImageBoundImportDescriptor!()