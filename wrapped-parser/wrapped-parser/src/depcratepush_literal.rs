// Generated macro for push_literal (function)
macro_rules! Depcratepush_literal {
() => {
// Module: crate
// Provides: {"push_literal"}
// Dependencies: {}
fn push_literal < 'f > (frag : & mut Vec < Fragment < 'f > > , unescaped_literal : & 'f str) -> Result < () , Error > { let mut last_open = false ; let mut last_close = false ; for c in unescaped_literal . chars () { match c { '{' => last_open = ! last_open , '}' => last_close = ! last_close , _ if last_open => return Err (Error :: UnmatchedOpenBracket) , _ if last_close => return Err (Error :: UnmatchedCloseBracket) , _ => { } } } if last_open { return Err (Error :: UnmatchedOpenBracket) ; } else if last_close { return Err (Error :: UnmatchedCloseBracket) ; } let literal = unescaped_literal . replace ("{{" , "{") . replace ("}}" , "}") ; frag . push (Fragment :: Literal (literal . into ())) ; Ok (()) }
};
}
