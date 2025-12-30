// Generated macro for flatten_to_map (function)
macro_rules! Depcrate_mapflatten_to_map {
() => {
// Module: crate::map
// Provides: {"flatten_to_map"}
// Dependencies: {}
# [doc = "\nFlatten the fields of a value onto a map.\n\nThe `offset` is the current length of the map. A new offset will be returned\nwith the length of the map after flattening the value.\n"] pub fn flatten_to_map < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl sval :: Value + ? Sized) , offset : isize ,) -> sval :: Result < isize > { let stream = PassThru :: new (stream) ; let mut stream = Flattener :: begin (MapFlatten { stream } , offset) ; value . stream (& mut stream) ? ; Ok (stream . end ()) }
};
}
