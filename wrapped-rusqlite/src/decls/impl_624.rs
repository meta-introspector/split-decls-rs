macro_rules! deps {
    () => {
        CreateVTab!();
        CsvTab!();
        VTabKind!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl CreateVTab < '_ > for CsvTab { const KIND : VTabKind = VTabKind :: Default ; }
    };
}

impl_624!()