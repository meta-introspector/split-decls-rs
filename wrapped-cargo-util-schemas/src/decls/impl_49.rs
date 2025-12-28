macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
        SourceKind!();
        TomlLockfileSourceIdError!();
        GitReference!();
        TomlLockfileSourceIdErrorKind!();
        Result!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl TomlLockfileSourceId { pub fn new (source : String) -> Result < Self , TomlLockfileSourceIdError > { let source_str = source . clone () ; let (kind , url) = source . split_once ('+') . ok_or_else (| | TomlLockfileSourceIdErrorKind :: InvalidSource (source . clone ())) ? ; let url = Url :: parse (if kind == "sparse" { & source } else { url }) . map_err (| msg | { TomlLockfileSourceIdErrorKind :: InvalidUrl { url : url . to_string () , msg : msg . to_string () , } }) ? ; let kind = match kind { "git" => { let reference = GitReference :: from_query (url . query_pairs ()) ; SourceKind :: Git (reference) } "registry" => SourceKind :: Registry , "sparse" => SourceKind :: SparseRegistry , "path" => SourceKind :: Path , kind => { return Err (TomlLockfileSourceIdErrorKind :: UnsupportedSource (kind . to_string ()) . into () ,) ; } } ; Ok (Self { source_str , kind , url , }) } pub fn kind (& self) -> & SourceKind { & self . kind } pub fn url (& self) -> & Url { & self . url } pub fn source_str (& self) -> & String { & self . source_str } pub fn as_url (& self) -> impl fmt :: Display + '_ { self . source_str . clone () } }
    };
}

impl_49!();