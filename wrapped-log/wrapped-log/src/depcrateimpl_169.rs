// Generated macro for impl_169 (impl)
macro_rules! Depcrateimpl_169 {
() => {
// Module: crate
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a > MetadataBuilder < 'a > { # [doc = " Construct a new `MetadataBuilder`."] # [doc = ""] # [doc = " The default options are:"] # [doc = ""] # [doc = " - `level`: `Level::Info`"] # [doc = " - `target`: `\"\"`"] # [inline] pub fn new () -> MetadataBuilder < 'a > { MetadataBuilder { metadata : Metadata { level : Level :: Info , target : "" , } , } } # [doc = " Setter for [`level`](struct.Metadata.html#method.level)."] # [inline] pub fn level (& mut self , arg : Level) -> & mut MetadataBuilder < 'a > { self . metadata . level = arg ; self } # [doc = " Setter for [`target`](struct.Metadata.html#method.target)."] # [inline] pub fn target (& mut self , target : & 'a str) -> & mut MetadataBuilder < 'a > { self . metadata . target = target ; self } # [doc = " Returns a `Metadata` object."] # [inline] pub fn build (& self) -> Metadata < 'a > { self . metadata . clone () } }
};
}
