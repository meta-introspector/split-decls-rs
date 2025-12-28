macro_rules! deps {
    () => {
        ImportAlias!();
    };
}

macro_rules! ImportAliasDisplay {
    () => {
        deps!();
        struct ImportAliasDisplay < 'a > { value : & 'a ImportAlias , edition : Edition , }
    };
}

ImportAliasDisplay!();