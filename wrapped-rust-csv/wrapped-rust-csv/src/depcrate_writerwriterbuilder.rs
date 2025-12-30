// Generated macro for WriterBuilder (struct)
macro_rules! Depcrate_writerWriterBuilder {
() => {
// Module: crate::writer
// Provides: {"WriterBuilder"}
// Dependencies: {}
# [doc = " Builds a CSV writer with various configuration knobs."] # [doc = ""] # [doc = " This builder can be used to tweak the field delimiter, record terminator"] # [doc = " and more. Once a CSV `Writer` is built, its configuration cannot be"] # [doc = " changed."] # [derive (Debug)] pub struct WriterBuilder { builder : CoreWriterBuilder , capacity : usize , flexible : bool , has_headers : bool , }
};
}
