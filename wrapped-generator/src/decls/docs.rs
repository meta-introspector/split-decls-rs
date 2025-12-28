macro_rules! docs {
    () => {
        # [cfg (not (feature = "export-internal"))] mod docs ;
    };
}

docs!()