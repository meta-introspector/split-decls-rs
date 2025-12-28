macro_rules! deps {
    () => {
        References!();
    };
}

macro_rules! ReferenceNames {
    () => {
        deps!();
        # [doc = " An iterator over the names of references in a repository."] pub struct ReferenceNames < 'repo , 'references > { inner : & 'references mut References < 'repo > , }
    };
}

ReferenceNames!()