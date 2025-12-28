macro_rules! deps {
    () => {
        FrameNote!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl Subdiagnostic for FrameNote { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("times" , self . times) ; diag . arg ("where_" , self . where_) ; diag . arg ("instance" , self . instance) ; let mut span : MultiSpan = self . span . into () ; if self . has_label && ! self . span . is_dummy () { span . push_span_label (self . span , fluent :: const_eval_frame_note_last) ; } let msg = diag . eagerly_translate (fluent :: const_eval_frame_note) ; diag . remove_arg ("times") ; diag . remove_arg ("where_") ; diag . remove_arg ("instance") ; diag . span_note (span , msg) ; } }
    };
}

impl_175!()