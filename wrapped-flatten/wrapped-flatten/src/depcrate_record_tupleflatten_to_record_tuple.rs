// Generated macro for flatten_to_record_tuple (function)
macro_rules! Depcrate_record_tupleflatten_to_record_tuple {
() => {
// Module: crate::record_tuple
// Provides: {"flatten_to_record_tuple"}
// Dependencies: {}
# [doc = "\nFlatten the fields of a value onto a record or tuple.\n\nThe `offset` is the current length of the record or tuple. A new offset will be returned\nwith the length of the record or tuple after flattening the value.\n "] pub fn flatten_to_record_tuple < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl sval :: Value + ? Sized) , offset : isize ,) -> sval :: Result < isize > { let label_stream = LabelBuf :: default () ; let mut stream = Flattener :: begin (RecordTupleFlatten { stream , label_stream , } , offset ,) ; value . stream (& mut stream) ? ; Ok (stream . end ()) }
};
}
