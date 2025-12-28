macro_rules! deps {
    () => {
        BuildError!();
        PatternID!();
        BuildErrorKind!();
        NFA!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl BuildError { # [doc = " If it is known which pattern ID caused this build error to occur, then"] # [doc = " this method returns it."] # [doc = ""] # [doc = " Some errors are not associated with a particular pattern. However, any"] # [doc = " errors that occur as part of parsing a pattern are guaranteed to be"] # [doc = " associated with a pattern ID."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{meta::Regex, PatternID};"] # [doc = ""] # [doc = " let err = Regex::new_many(&[\"a\", \"b\", r\"\\p{Foo}\", \"c\"]).unwrap_err();"] # [doc = " assert_eq!(Some(PatternID::must(2)), err.pattern());"] # [doc = " ```"] pub fn pattern (& self) -> Option < PatternID > { match self . kind { BuildErrorKind :: Syntax { pid , .. } => Some (pid) , _ => None , } } # [doc = " If this error occurred because the regex exceeded the configured size"] # [doc = " limit before being built, then this returns the configured size limit."] # [doc = ""] # [doc = " The limit returned is what was configured, and corresponds to the"] # [doc = " maximum amount of heap usage in bytes."] pub fn size_limit (& self) -> Option < usize > { match self . kind { BuildErrorKind :: NFA (ref err) => err . size_limit () , _ => None , } } # [doc = " If this error corresponds to a syntax error, then a reference to it is"] # [doc = " returned by this method."] pub fn syntax_error (& self) -> Option < & regex_syntax :: Error > { match self . kind { BuildErrorKind :: Syntax { ref err , .. } => Some (err) , _ => None , } } pub (crate) fn ast (pid : PatternID , err : ast :: Error) -> BuildError { let err = regex_syntax :: Error :: from (err) ; BuildError { kind : BuildErrorKind :: Syntax { pid , err } } } pub (crate) fn hir (pid : PatternID , err : hir :: Error) -> BuildError { let err = regex_syntax :: Error :: from (err) ; BuildError { kind : BuildErrorKind :: Syntax { pid , err } } } pub (crate) fn nfa (err : nfa :: thompson :: BuildError) -> BuildError { BuildError { kind : BuildErrorKind :: NFA (err) } } }
    };
}

impl_303!();