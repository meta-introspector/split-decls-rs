macro_rules! miri {
    () => {
        # [cfg (miri)] mod miri ;
    };
}

miri!();