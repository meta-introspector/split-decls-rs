macro_rules! deps {
    () => {
        UnsupportedLiteral!();
        UnsupportedLiteralReason!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for UnsupportedLiteral { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , match self . reason { UnsupportedLiteralReason :: Generic => { fluent :: attr_parsing_unsupported_literal_generic } UnsupportedLiteralReason :: CfgString => { fluent :: attr_parsing_unsupported_literal_cfg_string } UnsupportedLiteralReason :: CfgBoolean => { fluent :: attr_parsing_unsupported_literal_cfg_boolean } } ,) ; diag . span (self . span) ; diag . code (E0565) ; if self . is_bytestr { diag . span_suggestion (self . start_point_span , fluent :: attr_parsing_unsupported_literal_suggestion , "" , Applicability :: MaybeIncorrect ,) ; } diag } }
    };
}

impl_333!();