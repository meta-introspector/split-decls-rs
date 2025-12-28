macro_rules! SEG_UNIXSTACK {
    () => {
        # [doc = " the unix stack segment"] pub const SEG_UNIXSTACK : & str = "__UNIXSTACK" ;
    };
}

SEG_UNIXSTACK!();