// Generated macro for impl_16 (impl)
macro_rules! Depcrate_printerimpl_16 {
() => {
// Module: crate::printer
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , Writer > InlineWriter < 'a , Writer > where Writer : fmt :: Write , { fn new (f : & 'a mut Writer) -> Self { InlineWriter { f , style : Style :: new () , } } # [doc = " Push a new character into the buffer, specifying the style it should be written in."] fn write_with_style < T : Into < Style > > (& mut self , c : & char , style : T) -> fmt :: Result { let style = style . into () ; if style == self . style { write ! (self . f , "{}" , c) ? ; } else { self . style . fmt_suffix (self . f) ? ; style . fmt_prefix (self . f) ? ; write ! (self . f , "{}" , c) ? ; self . style = style ; } Ok (()) } # [doc = " Finish any existing style and reset to default state."] fn finish (& mut self) -> fmt :: Result { self . style . fmt_suffix (self . f) ? ; writeln ! (self . f) ? ; self . style = Style :: new () ; Ok (()) } }
};
}
