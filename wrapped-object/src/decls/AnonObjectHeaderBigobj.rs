macro_rules! deps {
    () => {
        U16!();
        U32!();
    };
}

macro_rules! AnonObjectHeaderBigobj {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AnonObjectHeaderBigobj { # [doc = " Must be IMAGE_FILE_MACHINE_UNKNOWN"] pub sig1 : U16 < LE > , # [doc = " Must be 0xffff"] pub sig2 : U16 < LE > , # [doc = " >= 2 (implies the Flags field is present)"] pub version : U16 < LE > , # [doc = " Actual machine - IMAGE_FILE_MACHINE_xxx"] pub machine : U16 < LE > , pub time_date_stamp : U32 < LE > , # [doc = " Must be `ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID`."] pub class_id : ClsId , # [doc = " Size of data that follows the header"] pub size_of_data : U32 < LE > , # [doc = " 0x1 -> contains metadata"] pub flags : U32 < LE > , # [doc = " Size of CLR metadata"] pub meta_data_size : U32 < LE > , # [doc = " Offset of CLR metadata"] pub meta_data_offset : U32 < LE > , # [doc = " extended from WORD"] pub number_of_sections : U32 < LE > , pub pointer_to_symbol_table : U32 < LE > , pub number_of_symbols : U32 < LE > , }
    };
}

AnonObjectHeaderBigobj!();