// Generated macro for flatten_to_record (function)
macro_rules! Depcrate_recordflatten_to_record {
() => {
// Module: crate::record
// Provides: {"flatten_to_record"}
// Dependencies: {}
# [doc = "\nFlatten the fields of a value onto a record.\n\nThe `offset` is the current length of the record. A new offset will be returned\nwith the length of the record after flattening the value.\n "] pub fn flatten_to_record < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl sval :: Value + ? Sized) , offset : isize ,) -> sval :: Result < isize > { let label_stream = LabelBuf :: default () ; let mut stream = Flattener :: begin (RecordFlatten { stream , label_stream , } , offset ,) ; value . stream (& mut stream) ? ; Ok (stream . end ()) }
};
}
