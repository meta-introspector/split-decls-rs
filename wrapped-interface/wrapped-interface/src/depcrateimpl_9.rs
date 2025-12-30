// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl syn :: parse :: Parse for Interface { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let attributes = input . call (syn :: Attribute :: parse_outer) ? ; let mut docs = Vec :: new () ; for attr in attributes . into_iter () { let path = attr . path () ; if path . is_ident ("doc") { docs . push (attr) ; } else { return Err (syn :: Error :: new (path . span () , "Unrecognized attribute ")) ; } } let visibility = input . parse :: < syn :: Visibility > () ? ; _ = input . parse :: < syn :: Token ! [unsafe] > () ? ; _ = input . parse :: < syn :: Token ! [trait] > () ? ; let name = input . parse :: < syn :: Ident > () ? ; _ = input . parse :: < syn :: Token ! [:] > () ; let parent = input . parse :: < syn :: Path > () . ok () ; let content ; syn :: braced ! (content in input) ; let mut methods = Vec :: new () ; while ! content . is_empty () { methods . push (content . parse :: < InterfaceMethod > () ?) ; } Ok (Self { visibility , methods , name , parent , docs , }) } }
};
}
