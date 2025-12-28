macro_rules! deps {
    () => {
        DecodeError!();
        Decoder!();
        Decode!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl < Context > Decode < Context > for Duration { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { const NANOS_PER_SEC : u64 = 1_000_000_000 ; let secs : u64 = Decode :: decode (decoder) ? ; let nanos : u32 = Decode :: decode (decoder) ? ; if secs . checked_add (u64 :: from (nanos) / NANOS_PER_SEC) . is_none () { return Err (DecodeError :: InvalidDuration { secs , nanos }) ; } Ok (Duration :: new (secs , nanos)) } }
    };
}

impl_381!();