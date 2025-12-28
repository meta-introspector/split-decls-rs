macro_rules! NonFmtPanicUnused {
    () => {
        pub (crate) struct NonFmtPanicUnused { pub count : usize , pub suggestion : Option < Span > , }
    };
}

NonFmtPanicUnused!()