macro_rules! deps {
    () => {
        Section!();
        Table!();
    };
}

macro_rules! Sections {
    () => {
        deps!();
        # [doc = " A section table."] pub type Sections < 'data > = Table < Section < 'data > > ;
    };
}

Sections!();