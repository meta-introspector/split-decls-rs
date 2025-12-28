macro_rules! ONE_JEC {
    () => {
        # [doc = " Constant that can be added to add one to the JEC."] const ONE_JEC : usize = 1 << JEC_SHIFT ;
    };
}

ONE_JEC!()