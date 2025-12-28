macro_rules! deps {
    () => {
        VersionVec!();
    };
}

macro_rules! Access {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct Access { path_id : usize , dpor_vv : VersionVec , }
    };
}

Access!();