// Generated macro for convert (function)
macro_rules! Depcrateconvert {
() => {
// Module: crate
// Provides: {"convert"}
// Dependencies: {}
# [doc = " Converts the handlebars markdown file at the given path into the given"] # [doc = " format, returning the translated result."] pub fn convert (file : & Path , format : Format , url : Option < Url > , man_map : ManMap ,) -> Result < String , Error > { let formatter : Box < dyn Formatter + Send + Sync > = match format { Format :: Man => Box :: new (format :: man :: ManFormatter :: new (url)) , Format :: Md => Box :: new (format :: md :: MdFormatter :: new (man_map)) , Format :: Text => Box :: new (format :: text :: TextFormatter :: new (url)) , } ; let expanded = hbs :: expand (file , & * formatter) ? ; let expanded = expanded . replace ("\r\n" , "\n") ; formatter . render (& expanded) }
};
}
