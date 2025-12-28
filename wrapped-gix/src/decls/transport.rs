macro_rules! deps {
    () => {
        Error!();
        Http!();
        UnsignedInteger!();
        Boolean!();
    };
}

macro_rules! transport {
    () => {
        deps!();
        # [doc = ""] pub mod transport { use std :: borrow :: Cow ; use crate :: bstr :: BStr ; # [doc = " The error produced when configuring a transport for a particular protocol."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not interpret configuration key {key:?} as {kind} integer of desired range with value: {actual}")] InvalidInteger { key : & 'static str , kind : & 'static str , actual : i64 , } , # [error ("Could not interpret configuration key {key:?}")] ConfigValue { source : gix_config :: value :: Error , key : & 'static str , } , # [error ("Could not interpolate path at key {key:?}")] InterpolatePath { source : gix_config :: path :: interpolate :: Error , key : & 'static str , } , # [error ("Could not decode value at key {key:?} as UTF-8 string")] IllformedUtf8 { key : Cow < 'static , BStr > , source : crate :: config :: string :: Error , } , # [error ("Invalid URL passed for configuration")] ParseUrl (# [from] gix_url :: parse :: Error) , # [error ("Could obtain configuration for an HTTP url")] Http (# [from] http :: Error) , } # [doc = ""] pub mod http { use std :: borrow :: Cow ; use crate :: bstr :: BStr ; # [doc = " The error produced when configuring a HTTP transport."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Boolean (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] UnsignedInteger (# [from] crate :: config :: unsigned_integer :: Error) , # [error (transparent)] ConnectTimeout (# [from] crate :: config :: duration :: Error) , # [error ("The proxy authentication at key `{key}` is invalid")] InvalidProxyAuthMethod { source : crate :: config :: key :: GenericErrorWithValue , key : Cow < 'static , BStr > , } , # [error ("Could not configure the credential helpers for the authenticated proxy url")] # [cfg (feature = "credentials")] ConfigureProxyAuthenticate (# [from] crate :: config :: snapshot :: credential_helpers :: Error) , # [error (transparent)] InvalidSslVersion (# [from] crate :: config :: ssl_version :: Error) , # [error ("The HTTP version must be 'HTTP/2' or 'HTTP/1.1'")] InvalidHttpVersion (# [from] crate :: config :: key :: GenericErrorWithValue) , # [error ("The follow redirects value 'initial', or boolean true or false")] InvalidFollowRedirects (# [source] crate :: config :: key :: GenericErrorWithValue) , } } }
    };
}

transport!();