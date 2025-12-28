macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Diff {
    () => {
        deps!();
        # [doc = " The `diff` top-level section."] # [derive (Copy , Clone , Default)] # [cfg (feature = "blob-diff")] pub struct Diff ;
    };
}

Diff!()