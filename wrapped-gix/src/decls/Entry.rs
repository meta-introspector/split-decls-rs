macro_rules! deps {
    () => {
        Repository!();
        Tree!();
        Clone!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " An entry in a [`Tree`], similar to an entry in a directory."] # [derive (PartialEq , Debug , Clone)] pub struct Entry < 'repo > { pub (crate) inner : gix_object :: tree :: Entry , # [doc = " The owning repository."] pub repo : & 'repo crate :: Repository , }
    };
}

Entry!()