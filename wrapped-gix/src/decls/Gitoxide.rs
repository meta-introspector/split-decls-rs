macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Gitoxide {
    () => {
        deps!();
        # [doc = " The `gitoxide` top-level section."] # [derive (Copy , Clone , Default)] pub struct Gitoxide ;
    };
}

Gitoxide!()