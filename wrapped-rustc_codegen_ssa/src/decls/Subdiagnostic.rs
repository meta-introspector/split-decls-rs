macro_rules! Subdiagnostic {
    () => {
        pub (crate) struct Subdiagnostic { level : Level , messages : Vec < (DiagMessage , Style) > , }
    };
}

Subdiagnostic!()