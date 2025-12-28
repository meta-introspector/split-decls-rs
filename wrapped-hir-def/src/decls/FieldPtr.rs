macro_rules! FieldPtr {
    () => {
        pub type FieldPtr = AstPtr < ast :: RecordExprField > ;
    };
}

FieldPtr!();