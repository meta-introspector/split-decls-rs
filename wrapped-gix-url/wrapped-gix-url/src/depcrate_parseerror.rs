// Generated macro for Error (enum)
macro_rules! Depcrate_parseError {
() => {
// Module: crate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [parse()](crate::parse())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{} \"{url}\" is not valid UTF-8" , kind . as_str ())] Utf8 { url : BString , kind : UrlKind , source : std :: str :: Utf8Error , } , # [error ("{} {url:?} can not be parsed as valid URL" , kind . as_str ())] Url { url : String , kind : UrlKind , source : crate :: simple_url :: UrlParseError , } , # [error ("The host portion of the following URL is too long ({} bytes, {len} bytes total): {truncated_url:?}" , truncated_url . len ())] TooLong { truncated_url : BString , len : usize } , # [error ("{} \"{url}\" does not specify a path to a repository" , kind . as_str ())] MissingRepositoryPath { url : BString , kind : UrlKind } , # [error ("URL {url:?} is relative which is not allowed in this context")] RelativeUrl { url : String } , }
};
}
