macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < const CAP : usize > AsRef < Path > for ArrayString < CAP > { fn as_ref (& self) -> & Path { self . as_str () . as_ref () } }
    };
}

impl_20!();