// Generated macro for impl_85 (impl)
macro_rules! Depcrate_stream_xml_readerimpl_85 {
() => {
// Module: crate::stream::xml_reader
// Provides: {"impl_85"}
// Dependencies: {}
impl From < XmlReaderError > for ErrorKind { fn from (err : XmlReaderError) -> Self { match err { XmlReaderError :: Io (err) if err . kind () == io :: ErrorKind :: UnexpectedEof => { ErrorKind :: UnexpectedEof } XmlReaderError :: Io (err) => match std :: sync :: Arc :: try_unwrap (err) { Ok (err) => ErrorKind :: Io (err) , Err (err) => ErrorKind :: Io (std :: io :: Error :: from (err . kind ())) , } , XmlReaderError :: Syntax (_) => ErrorKind :: UnexpectedEof , XmlReaderError :: IllFormed (_) | XmlReaderError :: InvalidAttr (_) | XmlReaderError :: Escape (_) | XmlReaderError :: Namespace (_) => { ErrorKind :: InvalidXmlSyntax } , XmlReaderError :: Encoding (_) => ErrorKind :: InvalidXmlUtf8 , } } }
};
}
