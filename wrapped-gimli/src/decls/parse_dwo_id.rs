macro_rules! deps {
    () => {
        Reader!();
        Result!();
        DwoId!();
    };
}

macro_rules! parse_dwo_id {
    () => {
        deps!();
        # [doc = " Parse a dwo_id from a header"] fn parse_dwo_id < R : Reader > (input : & mut R) -> Result < DwoId > { Ok (DwoId (input . read_u64 () ?)) }
    };
}

parse_dwo_id!();