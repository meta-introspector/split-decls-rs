macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! EntryRef {
    () => {
        deps!();
        # [doc = " An entry within a tree"] pub struct EntryRef < 'repo , 'a > { # [doc = " The actual entry ref we are wrapping."] pub inner : gix_object :: tree :: EntryRef < 'a > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

EntryRef!()