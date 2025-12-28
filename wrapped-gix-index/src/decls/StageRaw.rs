macro_rules! StageRaw {
    () => {
        # [doc = " * 0 = no conflict,"] # [doc = " * 1 = base,"] # [doc = " * 2 = ours,"] # [doc = " * 3 = theirs"] pub type StageRaw = u32 ;
    };
}

StageRaw!()