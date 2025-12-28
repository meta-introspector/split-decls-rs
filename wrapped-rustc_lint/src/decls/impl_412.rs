macro_rules! deps {
    () => {
        BuiltinUnpermittedTypeInitSub!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl Subdiagnostic for BuiltinUnpermittedTypeInitSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut err = self . err ; loop { if let Some (span) = err . span { diag . span_note (span , err . message) ; } else { diag . note (err . message) ; } if let Some (e) = err . nested { err = * e ; } else { break ; } } } }
    };
}

impl_412!();