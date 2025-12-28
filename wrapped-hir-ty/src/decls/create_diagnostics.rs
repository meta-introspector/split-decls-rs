macro_rules! deps {
    () => {
        Diagnostics!();
    };
}

macro_rules! create_diagnostics {
    () => {
        deps!();
        pub (crate) fn create_diagnostics (diagnostics : Vec < TyLoweringDiagnostic >) -> Diagnostics { (! diagnostics . is_empty ()) . then (| | ThinArc :: from_header_and_iter (() , diagnostics . into_iter ())) }
    };
}

create_diagnostics!();