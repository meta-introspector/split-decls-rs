// Generated macro for did_you_mean (function)
macro_rules! Depcrate_error_formatdid_you_mean {
() => {
// Module: crate::error::format
// Provides: {"did_you_mean"}
// Dependencies: {}
# [cfg (feature = "error-context")] fn did_you_mean (styled : & mut StyledStr , styles : & Styles , context : & str , possibles : & ContextValue) { use std :: fmt :: Write as _ ; let valid = & styles . get_valid () ; let _ = write ! (styled , "{TAB}{valid}tip:{valid:#}" ,) ; if let ContextValue :: String (possible) = possibles { let _ = write ! (styled , " a similar {context} exists: '{valid}{possible}{valid:#}'" ,) ; } else if let ContextValue :: Strings (possibles) = possibles { if possibles . len () == 1 { let _ = write ! (styled , " a similar {context} exists: " ,) ; } else { let _ = write ! (styled , " some similar {context}s exist: " ,) ; } for (i , possible) in possibles . iter () . enumerate () { if i != 0 { styled . push_str (", ") ; } let _ = write ! (styled , "'{valid}{possible}{valid:#}'" ,) ; } } }
};
}
