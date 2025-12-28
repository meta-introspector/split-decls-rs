macro_rules! deps {
    () => {
        Version!();
        U32!();
    };
}

macro_rules! ImageHotPatchBase {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageHotPatchBase { pub sequence_number : U32 < LE > , pub flags : U32 < LE > , pub original_time_date_stamp : U32 < LE > , pub original_check_sum : U32 < LE > , pub code_integrity_info : U32 < LE > , pub code_integrity_size : U32 < LE > , pub patch_table : U32 < LE > , # [doc = " Version 2 and later"] pub buffer_offset : U32 < LE > , }
    };
}

ImageHotPatchBase!();