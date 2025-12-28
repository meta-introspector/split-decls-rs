macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Branch {
    () => {
        deps!();
        # [doc = " The `branch` top-level section."] # [derive (Copy , Clone , Default)] pub struct Branch ;
    };
}

Branch!();