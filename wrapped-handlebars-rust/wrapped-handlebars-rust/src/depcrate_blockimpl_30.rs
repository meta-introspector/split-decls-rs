// Generated macro for impl_30 (impl)
macro_rules! Depcrate_blockimpl_30 {
() => {
// Module: crate::block
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'reg > BlockParams < 'reg > { # [doc = " Create a empty block parameter map."] pub fn new () -> BlockParams < 'reg > { BlockParams :: default () } # [doc = " Add a path reference as the parameter. The `path` is a vector of path"] # [doc = " segments the relative to current block's base path."] pub fn add_path (& mut self , k : & 'reg str , path : Vec < String >) -> Result < () , RenderError > { self . data . insert (k , BlockParamHolder :: path (path)) ; Ok (()) } # [doc = " Add a value as parameter."] pub fn add_value (& mut self , k : & 'reg str , v : Json) -> Result < () , RenderError > { self . data . insert (k , BlockParamHolder :: value (v)) ; Ok (()) } # [doc = " Get a block parameter by its name."] pub fn get (& self , k : & str) -> Option < & BlockParamHolder > { self . data . get (k) } }
};
}
