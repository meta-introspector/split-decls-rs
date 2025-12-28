macro_rules! deps {
    () => {
        U32!();
        Version!();
    };
}

macro_rules! ImageHotPatchInfo {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageHotPatchInfo { pub version : U32 < LE > , pub size : U32 < LE > , pub sequence_number : U32 < LE > , pub base_image_list : U32 < LE > , pub base_image_count : U32 < LE > , # [doc = " Version 2 and later"] pub buffer_offset : U32 < LE > , # [doc = " Version 3 and later"] pub extra_patch_size : U32 < LE > , }
    };
}

ImageHotPatchInfo!()