macro_rules! DenyExplicitImplParser {
    () => {
        pub (crate) struct DenyExplicitImplParser ;
    };
}

DenyExplicitImplParser!();