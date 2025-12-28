macro_rules! DICT_SIZE_MAX {
    () => {
        # [doc = " The maximal size of a dictionary."] pub const DICT_SIZE_MAX : u32 = u32 :: MAX & ! 15_u32 ;
    };
}

DICT_SIZE_MAX!()