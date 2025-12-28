macro_rules! coff {
    () => {
        # [cfg (feature = "coff")] pub mod coff ;
    };
}

coff!()