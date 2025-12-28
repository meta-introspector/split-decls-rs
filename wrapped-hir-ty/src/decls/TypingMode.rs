macro_rules! TypingMode {
    () => {
        pub type TypingMode < 'db > = rustc_type_ir :: TypingMode < DbInterner < 'db > > ;
    };
}

TypingMode!()