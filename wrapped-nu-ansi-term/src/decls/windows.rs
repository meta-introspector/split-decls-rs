macro_rules! windows {
    () => {
        # [cfg (all (windows , feature = "std"))] mod windows ;
    };
}

windows!()