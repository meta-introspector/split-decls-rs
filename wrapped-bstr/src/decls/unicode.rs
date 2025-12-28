macro_rules! unicode {
    () => {
        # [cfg (feature = "unicode")] mod unicode ;
    };
}

unicode!()