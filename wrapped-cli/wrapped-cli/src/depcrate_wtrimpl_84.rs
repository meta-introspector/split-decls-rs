// Generated macro for impl_84 (impl)
macro_rules! Depcrate_wtrimpl_84 {
() => {
// Module: crate::wtr
// Provides: {"impl_84"}
// Dependencies: {}
impl termcolor :: WriteColor for StandardStream { # [inline] fn supports_color (& self) -> bool { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref w) => w . supports_color () , BlockBuffered (ref w) => w . supports_color () , } } # [inline] fn supports_hyperlinks (& self) -> bool { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref w) => w . supports_hyperlinks () , BlockBuffered (ref w) => w . supports_hyperlinks () , } } # [inline] fn set_color (& mut self , spec : & termcolor :: ColorSpec) -> io :: Result < () > { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref mut w) => w . set_color (spec) , BlockBuffered (ref mut w) => w . set_color (spec) , } } # [inline] fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref mut w) => w . set_hyperlink (link) , BlockBuffered (ref mut w) => w . set_hyperlink (link) , } } # [inline] fn reset (& mut self) -> io :: Result < () > { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref mut w) => w . reset () , BlockBuffered (ref mut w) => w . reset () , } } # [inline] fn is_synchronous (& self) -> bool { use self :: StandardStreamKind :: * ; match self . 0 { LineBuffered (ref w) => w . is_synchronous () , BlockBuffered (ref w) => w . is_synchronous () , } } }
};
}
