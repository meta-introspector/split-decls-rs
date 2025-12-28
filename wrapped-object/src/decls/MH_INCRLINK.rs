macro_rules! MH_INCRLINK {
    () => {
        # [doc = " the object file is the output of an incremental link against a base file and can't be link edited again"] pub const MH_INCRLINK : u32 = 0x2 ;
    };
}

MH_INCRLINK!()