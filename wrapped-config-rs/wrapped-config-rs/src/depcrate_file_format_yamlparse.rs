// Generated macro for parse (function)
macro_rules! Depcrate_file_format_yamlparse {
() => {
// Module: crate::file::format::yaml
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (uri : Option < & String > , text : & str ,) -> Result < Map < String , Value > , Box < dyn Error + Send + Sync > > { let mut docs = yaml :: YamlLoader :: load_from_str (text) ? ; let root = match docs . len () { 0 => yaml :: Yaml :: Hash (yaml :: yaml :: Hash :: new ()) , 1 => mem :: replace (& mut docs [0] , yaml :: Yaml :: Null) , n => { return Err (Box :: new (MultipleDocumentsError (n))) ; } } ; let value = from_yaml_value (uri , & root) ? ; format :: extract_root_table (uri , value) }
};
}
