macro_rules! weak {
    () => {
        # [cfg (feature = "weak")] mod weak ;
    };
}

weak!()