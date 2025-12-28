macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! User {
    () => {
        deps!();
        # [doc = " The `user` top-level section."] # [derive (Copy , Clone , Default)] pub struct User ;
    };
}

User!()