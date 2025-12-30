// Generated macro for ReaderInner (enum)
macro_rules! Depcrate_streamReaderInner {
() => {
// Module: crate::stream
// Provides: {"ReaderInner"}
// Dependencies: {}
enum ReaderInner < R : Read + Seek > { Uninitialized (Option < R >) , Binary (BinaryReader < R >) , Xml (XmlReader < BufReader < R > >) , Ascii (AsciiReader < BufReader < R > >) , }
};
}
