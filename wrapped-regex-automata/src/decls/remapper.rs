macro_rules! remapper {
    () => {
        # [cfg (any (feature = "dfa-build" , feature = "dfa-onepass"))] mod remapper ;
    };
}

remapper!();