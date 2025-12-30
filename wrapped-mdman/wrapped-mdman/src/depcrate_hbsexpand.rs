// Generated macro for expand (function)
macro_rules! Depcrate_hbsexpand {
() => {
// Module: crate::hbs
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Processes the handlebars template at the given file."] pub fn expand (file : & Path , formatter : FormatterRef < '_ >) -> Result < String , Error > { let mut handlebars = Handlebars :: new () ; handlebars . set_strict_mode (true) ; handlebars . register_helper ("lower" , Box :: new (lower)) ; handlebars . register_helper ("options" , Box :: new (OptionsHelper { formatter })) ; handlebars . register_helper ("option" , Box :: new (OptionHelper { formatter })) ; handlebars . register_helper ("man" , Box :: new (ManLinkHelper { formatter })) ; handlebars . register_decorator ("set" , Box :: new (set_decorator)) ; handlebars . register_template_file ("template" , file) ? ; let includes = file . parent () . unwrap () . join ("includes") ; let mut options = DirectorySourceOptions :: default () ; options . tpl_extension = ".md" . to_string () ; handlebars . register_templates_directory (includes , options) ? ; let man_name = file . file_stem () . expect ("expected filename") . to_str () . expect ("utf8 filename") . to_string () ; let data = HashMap :: from ([("man_name" , man_name)]) ; let expanded = handlebars . render ("template" , & data) ? ; Ok (expanded) }
};
}
