// Generated macro for flatten_to_seq (function)
macro_rules! Depcrate_seqflatten_to_seq {
() => {
// Module: crate::seq
// Provides: {"flatten_to_seq"}
// Dependencies: {}
# [doc = "\nFlatten the fields of a value onto a sequence.\n "] pub fn flatten_to_seq < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl sval :: Value + ? Sized) ,) -> sval :: Result { let label_stream = Empty ; let mut stream = Flattener :: begin (SeqFlatten { stream , label_stream , } , 0 ,) ; value . stream (& mut stream) ? ; Ok (()) }
};
}
