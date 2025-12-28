macro_rules! deps {
    () => {
        Subdiagnostic!();
    };
}

macro_rules! Diagnostic {
    () => {
        deps!();
        struct Diagnostic { level : Level , messages : Vec < (DiagMessage , Style) > , code : Option < ErrCode > , children : Vec < Subdiagnostic > , args : DiagArgMap , }
    };
}

Diagnostic!();