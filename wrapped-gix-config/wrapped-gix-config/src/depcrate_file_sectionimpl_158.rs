// Generated macro for impl_158 (impl)
macro_rules! Depcrate_file_sectionimpl_158 {
() => {
// Module: crate::file::section
// Provides: {"impl_158"}
// Dependencies: {}
# [doc = " Instantiation and conversion"] impl < 'a > Section < 'a > { # [doc = " Create a new section with the given `name` and optional, `subsection`, `meta`-data and an empty body."] pub fn new (name : impl Into < Cow < 'a , str > > , subsection : impl Into < Option < Cow < 'a , BStr > > > , meta : impl Into < OwnShared < file :: Metadata > > ,) -> Result < Self , parse :: section :: header :: Error > { Ok (Section { header : parse :: section :: Header :: new (name , subsection) ? , body : Default :: default () , meta : meta . into () , id : SectionId :: default () , }) } }
};
}
