macro_rules! PatFieldPtr {
    () => {
        pub type PatFieldPtr = AstPtr < Either < ast :: RecordExprField , ast :: RecordPatField > > ;
    };
}

PatFieldPtr!()