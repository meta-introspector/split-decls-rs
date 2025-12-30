// Generated macro for impl_103 (impl)
macro_rules! Depcrate_stream_xml_writerimpl_103 {
() => {
// Module: crate::stream::xml_writer
// Provides: {"impl_103"}
// Dependencies: {}
impl From < XmlWriterError > for Error { fn from (err : XmlWriterError) -> Self { match err { XmlWriterError :: Io (err) => match std :: sync :: Arc :: try_unwrap (err) { Ok (err) => ErrorKind :: Io (err) , Err (err) => ErrorKind :: Io (std :: io :: Error :: from (err . kind ())) , } . without_position () , _ => unreachable ! () , } } }
};
}
