macro_rules! Repository {
    () => {
        # [doc = " See [`new`]"] pub struct Repository (git2 :: Repository) ;
    };
}

Repository!()