// Generated macro for parse_source (function)
macro_rules! Depcrateparse_source {
() => {
// Module: crate
// Provides: {"parse_source"}
// Dependencies: {}
# [doc = " Parse the source"] fn parse_source (source : & Option < Source > , features : & [String] ,) -> anyhow :: Result < (Resolve , Vec < PackageId > , Vec < PathBuf >) > { let mut resolve = Resolve :: default () ; resolve . features . extend (features . iter () . cloned ()) ; let mut files = Vec :: new () ; let mut pkgs = Vec :: new () ; let root = PathBuf :: from (std :: env :: var ("CARGO_MANIFEST_DIR") . unwrap ()) ; let mut parse = | paths : & [PathBuf] | -> anyhow :: Result < () > { for path in paths { let p = root . join (path) ; let normalized_path = match std :: fs :: canonicalize (& p) { Ok (p) => p , Err (_) => p . to_path_buf () , } ; let (pkg , sources) = resolve . push_path (normalized_path) ? ; pkgs . push (pkg) ; files . extend (sources . paths () . map (| p | p . to_owned ())) ; } Ok (()) } ; let default = root . join ("wit") ; match source { Some (Source :: Inline (s , path)) => { match path { Some (p) => parse (p) ? , None => { if default . exists () { parse (& [default]) ? ; } } } pkgs . truncate (0) ; pkgs . push (resolve . push_group (UnresolvedPackageGroup :: parse ("macro-input" , s) ?) ?) ; } Some (Source :: Paths (p)) => parse (p) ? , None => parse (& [default]) ? , } ; Ok ((resolve , pkgs , files)) }
};
}
