// Generated macro for flatten_to_tuple (function)
macro_rules! Depcrate_tupleflatten_to_tuple {
() => {
// Module: crate::tuple
// Provides: {"flatten_to_tuple"}
// Dependencies: {}
# [doc = "\nFlatten the fields of a value onto a tuple.\n\nThe `offset` is the current length of the tuple. A new offset will be returned\nwith the length of the tuple after flattening the value.\n "] pub fn flatten_to_tuple < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl sval :: Value + ? Sized) , offset : isize ,) -> sval :: Result < isize > { let label_stream = Empty ; let mut stream = Flattener :: begin (TupleFlatten { stream , label_stream , } , offset ,) ; value . stream (& mut stream) ? ; Ok (stream . end ()) }
};
}
