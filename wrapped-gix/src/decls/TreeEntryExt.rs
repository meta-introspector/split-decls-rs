macro_rules! deps {
    () => {
        Entry!();
        Extensions!();
        Repository!();
    };
}

macro_rules! TreeEntryExt {
    () => {
        deps!();
        # [doc = " Extensions for [Entry](gix_object::tree::Entry)."] pub trait TreeEntryExt { # [doc = " Attach [`repo`](crate::Repository) to the given tree entry. It can be detached later with `detach()`."] fn attach (self , repo : & crate :: Repository) -> crate :: object :: tree :: Entry < '_ > ; }
    };
}

TreeEntryExt!();