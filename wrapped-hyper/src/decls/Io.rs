macro_rules! deps {
    () => {
        Write!();
        Read!();
    };
}

macro_rules! Io {
    () => {
        deps!();
        pub (super) trait Io : Read + Write + Unpin + 'static { fn __hyper_type_id (& self) -> TypeId { TypeId :: of :: < Self > () } }
    };
}

Io!();