macro_rules! BuiltLint {
    () => {
        struct BuiltLint { lint : & 'static Lint , groups : Vec < & 'static str > , }
    };
}

BuiltLint!()