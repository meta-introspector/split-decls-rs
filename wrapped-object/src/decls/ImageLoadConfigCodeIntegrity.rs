macro_rules! deps {
    () => {
        U32!();
        U16!();
    };
}

macro_rules! ImageLoadConfigCodeIntegrity {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageLoadConfigCodeIntegrity { # [doc = " Flags to indicate if CI information is available, etc."] pub flags : U16 < LE > , # [doc = " 0xFFFF means not available"] pub catalog : U16 < LE > , pub catalog_offset : U32 < LE > , # [doc = " Additional bitmask to be defined later"] pub reserved : U32 < LE > , }
    };
}

ImageLoadConfigCodeIntegrity!()