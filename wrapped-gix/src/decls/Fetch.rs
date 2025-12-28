macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Fetch {
    () => {
        deps!();
        # [doc = " The `fetch` top-level section."] # [derive (Copy , Clone , Default)] pub struct Fetch ;
    };
}

Fetch!();