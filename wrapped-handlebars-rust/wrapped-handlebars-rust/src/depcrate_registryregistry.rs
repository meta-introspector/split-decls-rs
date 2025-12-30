// Generated macro for Registry (struct)
macro_rules! Depcrate_registryRegistry {
() => {
// Module: crate::registry
// Provides: {"Registry"}
// Dependencies: {}
# [doc = " The single entry point of your Handlebars templates"] # [doc = ""] # [doc = " It maintains compiled templates and registered helpers."] # [derive (Clone)] pub struct Registry < 'reg > { templates : HashMap < String , Template > , helpers : HashMap < String , Arc < dyn HelperDef + Send + Sync + 'reg > > , decorators : HashMap < String , Arc < dyn DecoratorDef + Send + Sync + 'reg > > , escape_fn : EscapeFn , strict_mode : bool , dev_mode : bool , prevent_indent : bool , # [cfg (feature = "script_helper")] pub (crate) engine : Arc < Engine > , template_sources : HashMap < String , Arc < dyn Source < Item = String , Error = IoError > + Send + Sync + 'reg > > , # [cfg (feature = "script_helper")] script_sources : HashMap < String , Arc < dyn Source < Item = String , Error = IoError > + Send + Sync + 'reg > > , }
};
}
