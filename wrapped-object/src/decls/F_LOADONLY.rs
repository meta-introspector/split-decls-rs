macro_rules! F_LOADONLY {
    () => {
        # [doc = " If the object file is a member of an archive, it can be loaded by the system"] # [doc = " loader, but the member is ignored by the binder. If the object file is not in"] # [doc = " an archive, this flag has no effect."] pub const F_LOADONLY : u16 = 0x4000 ;
    };
}

F_LOADONLY!();