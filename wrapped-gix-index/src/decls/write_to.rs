macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! write_to {
    () => {
        deps!();
        # [doc = " Serialize the sparse index extension to `out`"] pub fn write_to (mut out : impl std :: io :: Write) -> Result < () , std :: io :: Error > { out . write_all (& SIGNATURE) ? ; out . write_all (& 0_u32 . to_be_bytes ()) ? ; Ok (()) }
    };
}

write_to!();