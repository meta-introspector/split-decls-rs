macro_rules! deps {
    () => {
        BufferedDiag!();
        BorrowckDiagnosticsBuffer!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'infcx , 'tcx > BorrowckDiagnosticsBuffer < 'infcx , 'tcx > { pub (crate) fn buffer_non_error (& mut self , diag : Diag < 'infcx , () >) { self . buffered_diags . push (BufferedDiag :: NonError (diag)) ; } }
    };
}

impl_168!()