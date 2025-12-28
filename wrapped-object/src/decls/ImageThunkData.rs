macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! ImageThunkData {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`pe::ImageThunkData32`] and [`pe::ImageThunkData64`]."] # [allow (missing_docs)] pub trait ImageThunkData : Debug + Pod { # [doc = " Return the raw thunk value."] fn raw (self) -> u64 ; # [doc = " Returns true if the ordinal flag is set."] fn is_ordinal (self) -> bool ; # [doc = " Return the ordinal portion of the thunk."] # [doc = ""] # [doc = " Does not check the ordinal flag."] fn ordinal (self) -> u16 ; # [doc = " Return the RVA portion of the thunk."] # [doc = ""] # [doc = " Does not check the ordinal flag."] fn address (self) -> u32 ; }
    };
}

ImageThunkData!();