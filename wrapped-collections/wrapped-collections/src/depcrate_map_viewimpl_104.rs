// Generated macro for impl_104 (impl)
macro_rules! Depcrate_map_viewimpl_104 {
() => {
// Module: crate::map_view
// Provides: {"impl_104"}
// Dependencies: {}
impl < K , V > IIterator_Impl < IKeyValuePair < K , V > > for StockMapViewIterator_Impl < '_ , K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn Current (& self) -> Result < IKeyValuePair < K , V > > { let mut current = self . current . read () . unwrap () . clone () . peekable () ; if let Some ((key , value)) = current . peek () { Ok (ComObject :: new (StockKeyValuePair { key : (* key) . clone () , value : (* value) . clone () , }) . into_interface ()) } else { Err (Error :: from (E_BOUNDS)) } } fn HasCurrent (& self) -> Result < bool > { let mut current = self . current . read () . unwrap () . clone () . peekable () ; Ok (current . peek () . is_some ()) } fn MoveNext (& self) -> Result < bool > { let mut current = self . current . write () . unwrap () ; current . next () ; Ok (current . clone () . peekable () . peek () . is_some ()) } fn GetMany (& self , pairs : & mut [Option < IKeyValuePair < K , V > >]) -> Result < u32 > { let mut current = self . current . write () . unwrap () ; let mut actual = 0 ; for pair in pairs { if let Some ((key , value)) = current . next () { * pair = Some (ComObject :: new (StockKeyValuePair { key : (* key) . clone () , value : (* value) . clone () , }) . into_interface () ,) ; actual += 1 ; } else { break ; } } Ok (actual) } }
};
}
