macro_rules! deps {
    () => {
        GemConfig!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl GemConfig { pub fn load_from_file (path : & std :: path :: Path) -> Result < Self > { let content = std :: fs :: read_to_string (path) ? ; let config : GemConfig = toml :: from_str (& content) ? ; Ok (config) } pub fn get_identifier_to_gem_map (& self) -> HashMap < String , String > { let mut map = HashMap :: new () ; for gem_entry in & self . gem { for identifier in & gem_entry . identifiers { map . insert (identifier . clone () , gem_entry . name . clone ()) ; } } map } }
    };
}

impl_87!()