// Generated macro for Writer (struct)
macro_rules! Depcrate_writerWriter {
() => {
// Module: crate::writer
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " An already configured CSV writer."] # [doc = ""] # [doc = " A CSV writer takes as input Rust values and writes those values in a valid"] # [doc = " CSV format as output."] # [doc = ""] # [doc = " While CSV writing is considerably easier than parsing CSV, a proper writer"] # [doc = " will do a number of things for you:"] # [doc = ""] # [doc = " 1. Quote fields when necessary."] # [doc = " 2. Check that all records have the same number of fields."] # [doc = " 3. Write records with a single empty field correctly."] # [doc = " 4. Automatically serialize normal Rust types to CSV records. When that"] # [doc = "    type is a struct, a header row is automatically written corresponding"] # [doc = "    to the fields of that struct."] # [doc = " 5. Use buffering intelligently and otherwise avoid allocation. (This means"] # [doc = "    that callers should not do their own buffering.)"] # [doc = ""] # [doc = " All of the above can be configured using a"] # [doc = " [`WriterBuilder`](struct.WriterBuilder.html)."] # [doc = " However, a `Writer` has a couple of convenience constructors (`from_path`"] # [doc = " and `from_writer`) that use the default configuration."] # [doc = ""] # [doc = " Note that the default configuration of a `Writer` uses `\\n` for record"] # [doc = " terminators instead of `\\r\\n` as specified by RFC 4180. Use the"] # [doc = " `terminator` method on `WriterBuilder` to set the terminator to `\\r\\n` if"] # [doc = " it's desired."] # [derive (Debug)] pub struct Writer < W : io :: Write > { core : CoreWriter , wtr : Option < W > , buf : Buffer , state : WriterState , }
};
}
