macro_rules! deps {
    () => {
        Table!();
        Version!();
    };
}

macro_rules! Versions {
    () => {
        deps!();
        # [doc = " A table of versions that are referenced by symbols."] pub type Versions < 'data > = Table < Version < 'data > > ;
    };
}

Versions!();