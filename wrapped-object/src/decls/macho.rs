macro_rules! macho {
    () => {
        # [cfg (feature = "macho")] pub mod macho ;
    };
}

macho!();