macro_rules! deps {
    () => {
        EmptyLabelManySpans!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Subdiagnostic for EmptyLabelManySpans { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_labels (self . 0 , "") ; } }
    };
}

impl_52!()