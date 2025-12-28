macro_rules! deps {
    () => {
        StyledStr!();
        ContextValue!();
        ErrorFormatter!();
        ContextKind!();
        RichFormatter!();
        Usage!();
        Error!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        # [cfg (feature = "error-context")] impl ErrorFormatter for RichFormatter { fn format_error (error : & crate :: error :: Error < Self >) -> StyledStr { use std :: fmt :: Write as _ ; let styles = & error . inner . styles ; let valid = & styles . get_valid () ; let mut styled = StyledStr :: new () ; start_error (& mut styled , styles) ; if ! write_dynamic_context (error , & mut styled , styles) { if let Some (msg) = error . kind () . as_str () { styled . push_str (msg) ; } else if let Some (source) = error . inner . source . as_ref () { let _ = write ! (styled , "{source}") ; } else { styled . push_str ("unknown cause") ; } } let mut suggested = false ; if let Some (valid) = error . get (ContextKind :: SuggestedSubcommand) { styled . push_str ("\n") ; if ! suggested { styled . push_str ("\n") ; suggested = true ; } did_you_mean (& mut styled , styles , "subcommand" , valid) ; } if let Some (valid) = error . get (ContextKind :: SuggestedArg) { styled . push_str ("\n") ; if ! suggested { styled . push_str ("\n") ; suggested = true ; } did_you_mean (& mut styled , styles , "argument" , valid) ; } if let Some (valid) = error . get (ContextKind :: SuggestedValue) { styled . push_str ("\n") ; if ! suggested { styled . push_str ("\n") ; suggested = true ; } did_you_mean (& mut styled , styles , "value" , valid) ; } let suggestions = error . get (ContextKind :: Suggested) ; if let Some (ContextValue :: StyledStrs (suggestions)) = suggestions { if ! suggested { styled . push_str ("\n") ; } for suggestion in suggestions { let _ = write ! (styled , "\n{TAB}{valid}tip:{valid:#} " ,) ; styled . push_styled (suggestion) ; } } let usage = error . get (ContextKind :: Usage) ; if let Some (ContextValue :: StyledStr (usage)) = usage { put_usage (& mut styled , usage) ; } try_help (& mut styled , styles , error . inner . help_flag . as_deref ()) ; styled } }
    };
}

impl_389!();