macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Branch {
    () => {
        deps!();
        # [doc = " The `branch` top-level section."] # [derive (Copy , Clone , Default)] pub struct Branch ;
    };
}

Branch!()