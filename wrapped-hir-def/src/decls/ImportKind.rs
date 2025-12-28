macro_rules! ImportKind {
    () => {
        # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum ImportKind { # [doc = " The `ModPath` is imported normally."] Plain , # [doc = " This is a glob-import of all names in the `ModPath`."] Glob , # [doc = " This is a `some::path::self` import, which imports `some::path` only in type namespace."] TypeOnly , }
    };
}

ImportKind!()