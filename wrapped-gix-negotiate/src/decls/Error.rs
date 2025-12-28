macro_rules! deps {
    () => {
        Negotiator!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " An error that happened during any of the methods on a [`Negotiator`]."] pub type Error = gix_revwalk :: graph :: get_or_insert_default :: Error ;
    };
}

Error!();