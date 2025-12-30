// Generated macro for isolated (module)
macro_rules! Depcrateisolated {
() => {
// Module: crate
// Provides: {"isolated"}
// Dependencies: {}
pub mod isolated { use partial_ref :: { part , PartialRefTarget } ; # [derive (Debug , PartialRefTarget)] # [part (PartC)] # [part (PartD)] pub struct Foo { # [part (PartA)] pub a : u32 , # [part (PartB)] pub b : u32 , } part ! (pub PartC) ; part ! (pub PartD) ; part ! (pub PartA <>: u32) ; part ! (pub PartB : u32) ; pub struct RefFoo < 'a > { pub a : & 'a mut u32 , } part ! (pub PartRefA <'a >: RefFoo <'a >) ; }
};
}
