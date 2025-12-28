macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Refdb {
    () => {
        deps!();
        # [doc = " This is used to logically indicate that a [`raw::git_reference`] or"] # [doc = " [`raw::git_reference_iterator`] holds a reference to [`raw::git_refdb`]."] # [doc = " It is not necessary to have a wrapper like this in the"] # [doc = " [`marker::PhantomData`], since all that matters is that it is tied to the"] # [doc = " lifetime of the [`Repository`], but this helps distinguish the actual"] # [doc = " references involved."] struct Refdb < 'repo > (# [allow (dead_code)] & 'repo Repository) ;
    };
}

Refdb!()