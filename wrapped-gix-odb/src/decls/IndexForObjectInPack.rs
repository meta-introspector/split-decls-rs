macro_rules! deps {
    () => {
        PackId!();
    };
}

macro_rules! IndexForObjectInPack {
    () => {
        deps!();
        pub struct IndexForObjectInPack { # [doc = " The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index."] pub (crate) pack_id : types :: PackId , # [doc = " The offset at which the object's entry can be found"] pub (crate) pack_offset : u64 , }
    };
}

IndexForObjectInPack!();