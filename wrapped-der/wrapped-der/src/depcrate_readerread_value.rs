// Generated macro for read_value (function)
macro_rules! Depcrate_readerread_value {
() => {
// Module: crate::reader
// Provides: {"read_value"}
// Dependencies: {}
# [doc = " Read a value (i.e. the \"V\" part of a \"TLV\" field) using the provided header."] # [doc = ""] # [doc = " This calls the provided function `f` with a nested reader created using"] # [doc = " [`Reader::read_nested`]."] pub (crate) fn read_value < 'r , R , T , F , E > (reader : & mut R , header : Header , f : F) -> Result < T , E > where R : Reader < 'r > , E : From < Error > , F : FnOnce (& mut R , Header) -> Result < T , E > , { # [cfg (feature = "ber")] let header = header . with_length (header . length () . sans_eoc ()) ; let ret = reader . read_nested (header . length () , | r | f (r , header)) ? ; # [cfg (feature = "ber")] if header . length () . is_indefinite () { read_eoc (reader) ? ; } Ok (ret) }
};
}
