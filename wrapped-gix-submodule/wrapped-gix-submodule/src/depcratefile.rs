// Generated macro for File (struct)
macro_rules! DepcrateFile {
() => {
// Module: crate
// Provides: {"File"}
// Dependencies: {}
# [doc = " All relevant information about a git module, typically from `.gitmodules` files."] # [doc = ""] # [doc = " Note that overrides from other configuration might be relevant, which is why this type"] # [doc = " can be used to take these into consideration when presented with other configuration"] # [doc = " from the superproject."] # [derive (Clone)] pub struct File { config : gix_config :: File < 'static > , }
};
}
