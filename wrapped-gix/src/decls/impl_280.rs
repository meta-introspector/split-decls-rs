macro_rules! deps {
    () => {
        Repository!();
        Reference!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'repo > Reference < 'repo > { pub (crate) fn from_ref (reference : gix_ref :: Reference , repo : & 'repo crate :: Repository) -> Self { Reference { inner : reference , repo } } }
    };
}

impl_280!()