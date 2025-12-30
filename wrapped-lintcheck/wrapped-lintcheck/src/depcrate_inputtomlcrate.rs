// Generated macro for TomlCrate (struct)
macro_rules! Depcrate_inputTomlCrate {
() => {
// Module: crate::input
// Provides: {"TomlCrate"}
// Dependencies: {}
# [doc = " A crate source stored inside the .toml"] # [doc = " will be translated into on one of the `CrateSource` variants"] # [derive (Debug , Deserialize)] struct TomlCrate { name : String , version : Option < String > , git_url : Option < String > , git_hash : Option < String > , path : Option < String > , options : Option < Vec < String > > , # [doc = " Magic values:"] # [doc = " * `{krate}` will be replaced by `self.name`"] # [doc = " * `{krate_}` will be replaced by `self.name` with all `-` replaced by `_`"] # [doc = " * `{version}` will be replaced by `self.version`"] # [doc = " * `{url}` will be replaced with `self.git_url`"] # [doc = " * `{hash}` will be replaced with `self.git_hash`"] # [doc = " * `{path}` will be replaced with `self.path`"] # [doc = " * `{file}` will be replaced by the path after `src/`"] # [doc = " * `{line}` will be replaced by the line"] # [doc = ""] # [doc = " If unset, this will be filled by [`read_crates`] since it depends on"] # [doc = " the source."] online_link : Option < String > , }
};
}
