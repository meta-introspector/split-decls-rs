macro_rules! ser {
    () => {
        # [cfg (not (feature = "std"))] mod ser ;
    };
}

ser!()