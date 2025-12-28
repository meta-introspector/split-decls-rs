macro_rules! deps {
    () => {
        ImportAliasDisplay!();
        ImportAlias!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl ImportAlias { pub fn display (& self , edition : Edition) -> impl fmt :: Display + '_ { ImportAliasDisplay { value : self , edition } } }
    };
}

impl_167!()