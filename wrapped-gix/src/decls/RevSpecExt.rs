macro_rules! deps {
    () => {
        Extensions!();
        Spec!();
        Repository!();
    };
}

macro_rules! RevSpecExt {
    () => {
        deps!();
        # [doc = " Extensions for [revision specifications][gix_revision::Spec]."] pub trait RevSpecExt { # [doc = " Attach [`Repository`][crate::Repository] to the given rev-spec."] fn attach (self , repo : & crate :: Repository) -> crate :: revision :: Spec < '_ > ; }
    };
}

RevSpecExt!();