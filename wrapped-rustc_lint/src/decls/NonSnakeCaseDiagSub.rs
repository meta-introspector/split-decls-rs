macro_rules! NonSnakeCaseDiagSub {
    () => {
        pub (crate) enum NonSnakeCaseDiagSub { Label { span : Span } , Help , RenameOrConvertSuggestion { span : Span , suggestion : Ident } , ConvertSuggestion { span : Span , suggestion : String } , SuggestionAndNote { span : Span } , }
    };
}

NonSnakeCaseDiagSub!()