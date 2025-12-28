macro_rules! RetInt {
    () => {
        # [cfg (not (target_pointer_width = "16"))] # [cfg (not (target_arch = "s390x"))] type RetInt = RegSize ;
    };
}

RetInt!()