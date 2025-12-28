macro_rules! deps {
    () => {
        U16!();
        U32!();
    };
}

macro_rules! ImageSeparateDebugHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageSeparateDebugHeader { pub signature : U16 < LE > , pub flags : U16 < LE > , pub machine : U16 < LE > , pub characteristics : U16 < LE > , pub time_date_stamp : U32 < LE > , pub check_sum : U32 < LE > , pub image_base : U32 < LE > , pub size_of_image : U32 < LE > , pub number_of_sections : U32 < LE > , pub exported_names_size : U32 < LE > , pub debug_directory_size : U32 < LE > , pub section_alignment : U32 < LE > , pub reserved : [U32 < LE > ; 2] , }
    };
}

ImageSeparateDebugHeader!();