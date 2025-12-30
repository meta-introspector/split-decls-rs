// Generated macro for construct (function)
macro_rules! Depcrateconstruct {
() => {
// Module: crate
// Provides: {"construct"}
// Dependencies: {}
fn construct (fields : & Fields , ctor : impl Fn (usize , & Field) -> Result < TokenStream > ,) -> Result < TokenStream > { let output = match fields { Fields :: Named (names) => { let names : Vec < TokenStream > = names . named . iter () . enumerate () . map (| (i , f) | { let name = f . ident . as_ref () . unwrap () ; ctor (i , f) . map (| ctor | quote ! { # name : # ctor }) }) . collect :: < Result < _ > > () ? ; quote ! { { # (# names ,) * } } } Fields :: Unnamed (names) => { let names : Vec < TokenStream > = names . unnamed . iter () . enumerate () . map (| (i , f) | ctor (i , f) . map (| ctor | quote ! { # ctor })) . collect :: < Result < _ > > () ? ; quote ! { (# (# names) ,*) } } Fields :: Unit => quote ! () , } ; Ok (output) }
};
}
