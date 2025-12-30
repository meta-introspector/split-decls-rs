// Generated macro for ReaderBuilder (struct)
macro_rules! Depcrate_readerReaderBuilder {
() => {
// Module: crate::reader
// Provides: {"ReaderBuilder"}
// Dependencies: {}
# [doc = " Builds a CSV reader with various configuration knobs."] # [doc = ""] # [doc = " This builder can be used to tweak the field delimiter, record terminator"] # [doc = " and more. Once a CSV `Reader` is built, its configuration cannot be"] # [doc = " changed."] # [derive (Debug)] pub struct ReaderBuilder { capacity : usize , flexible : bool , has_headers : bool , trim : Trim , # [doc = " The underlying CSV parser builder."] # [doc = ""] # [doc = " We explicitly put this on the heap because CoreReaderBuilder embeds an"] # [doc = " entire DFA transition table, which along with other things, tallies up"] # [doc = " to almost 500 bytes on the stack."] builder : Box < CoreReaderBuilder > , }
};
}
