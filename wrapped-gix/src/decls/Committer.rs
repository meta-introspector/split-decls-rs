macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Committer {
    () => {
        deps!();
        # [doc = " The `committer` top-level section."] # [derive (Copy , Clone , Default)] pub struct Committer ;
    };
}

Committer!()