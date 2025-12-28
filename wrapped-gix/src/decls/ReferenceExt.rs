macro_rules! deps {
    () => {
        Repository!();
        Reference!();
        Extensions!();
    };
}

macro_rules! ReferenceExt {
    () => {
        deps!();
        # [doc = " Extensions for [references][gix_ref::Reference]."] pub trait ReferenceExt { # [doc = " Attach [`Repository`][crate::Repository] to the given reference. It can be detached later with [`detach()]`."] fn attach (self , repo : & crate :: Repository) -> crate :: Reference < '_ > ; }
    };
}

ReferenceExt!()