macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! User {
    () => {
        deps!();
        # [doc = " The `user` top-level section."] # [derive (Copy , Clone , Default)] pub struct User ;
    };
}

User!();