macro_rules! deps {
    () => {
        Options!();
        Conflict!();
    };
}

macro_rules! with_extra_markers {
    () => {
        deps!();
        fn with_extra_markers (opts : & Options , extra_makers : u8) -> crate :: blob :: platform :: merge :: Options { let mut out = opts . blob_merge ; if let crate :: blob :: builtin_driver :: text :: Conflict :: Keep { marker_size , .. } = & mut out . text . conflict { * marker_size = marker_size . saturating_add (extra_makers . saturating_add (opts . marker_size_multiplier . saturating_mul (2))) ; } out }
    };
}

with_extra_markers!();