// Generated macro for parse (function)
macro_rules! Depcrate_file_format_iniparse {
() => {
// Module: crate::file::format::ini
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let mut map : Map < String , Value > = Map :: new () ; let i = Ini :: load_from_str (text) ? ; for (sec , prop) in i . iter () { match sec { Some (sec) => { let mut sec_map : Map < String , Value > = Map :: new () ; for (k , v) in prop . iter () { sec_map . insert (k . to_owned () , Value :: new (uri , ValueKind :: String (v . to_owned ())) ,) ; } map . insert (sec . to_owned () , Value :: new (uri , ValueKind :: Table (sec_map))) ; } None => { for (k , v) in prop . iter () { map . insert (k . to_owned () , Value :: new (uri , ValueKind :: String (v . to_owned ())) ,) ; } } } } Ok (map) }
};
}
