macro_rules! pe {
    () => {
        # [cfg (any (feature = "coff" , feature = "pe"))] pub mod pe ;
    };
}

pe!()