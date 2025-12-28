macro_rules! deps {
    () => {
        Error!();
        Scheme!();
    };
}

macro_rules! UrlParseError {
    () => {
        deps!();
        # [doc = " Minimal parse error type to replace url::ParseError"] # [derive (Debug , Clone , PartialEq , Eq , thiserror :: Error)] # [allow (missing_docs)] pub enum UrlParseError { # [error ("relative URL without a base")] RelativeUrlWithoutBase , # [error ("invalid port number - must be between 1-65535")] InvalidPort , # [error ("invalid domain character")] InvalidDomainCharacter , # [error ("Scheme requires host")] SchemeRequiresHost , }
    };
}

UrlParseError!();