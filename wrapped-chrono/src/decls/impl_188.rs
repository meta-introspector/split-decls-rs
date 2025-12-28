macro_rules! deps {
    () => {
        Utc!();
        DateTime!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < SystemTime > for DateTime < Utc > { fn from (t : SystemTime) -> DateTime < Utc > { let (sec , nsec) = match t . duration_since (UNIX_EPOCH) { Ok (dur) => (dur . as_secs () as i64 , dur . subsec_nanos ()) , Err (e) => { let dur = e . duration () ; let (sec , nsec) = (dur . as_secs () as i64 , dur . subsec_nanos ()) ; if nsec == 0 { (- sec , 0) } else { (- sec - 1 , 1_000_000_000 - nsec) } } } ; Utc . timestamp_opt (sec , nsec) . unwrap () } }
    };
}

impl_188!();