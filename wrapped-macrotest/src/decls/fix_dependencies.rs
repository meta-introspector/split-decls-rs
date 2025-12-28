macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! fix_dependencies {
    () => {
        deps!();
        fn fix_dependencies (dependencies : & mut Map < String , Dependency > , dir : & Path) { dependencies . remove ("macrotest") ; for dep in dependencies . values_mut () { dep . path = dep . path . as_ref () . map (| path | dir . join (path)) ; } }
    };
}

fix_dependencies!()