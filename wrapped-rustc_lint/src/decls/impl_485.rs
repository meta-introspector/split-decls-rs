macro_rules! deps {
    () => {
        NonSnakeCaseDiagSub!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl Subdiagnostic for NonSnakeCaseDiagSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { NonSnakeCaseDiagSub :: Label { span } => { diag . span_label (span , fluent :: lint_label) ; } NonSnakeCaseDiagSub :: Help => { diag . help (fluent :: lint_help) ; } NonSnakeCaseDiagSub :: ConvertSuggestion { span , suggestion } => { diag . span_suggestion (span , fluent :: lint_convert_suggestion , suggestion , Applicability :: MaybeIncorrect ,) ; } NonSnakeCaseDiagSub :: RenameOrConvertSuggestion { span , suggestion } => { diag . span_suggestion (span , fluent :: lint_rename_or_convert_suggestion , suggestion , Applicability :: MaybeIncorrect ,) ; } NonSnakeCaseDiagSub :: SuggestionAndNote { span } => { diag . note (fluent :: lint_cannot_convert_note) ; diag . span_suggestion (span , fluent :: lint_rename_suggestion , "" , Applicability :: MaybeIncorrect ,) ; } } } }
    };
}

impl_485!();