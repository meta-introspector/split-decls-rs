// Generated macro for PrinterPath (struct)
macro_rules! Depcrate_utilPrinterPath {
() => {
// Module: crate::util
// Provides: {"PrinterPath"}
// Dependencies: {}
# [doc = " A simple encapsulation of a file path used by a printer."] # [doc = ""] # [doc = " This represents any transforms that we might want to perform on the path,"] # [doc = " such as converting it to valid UTF-8 and/or replacing its separator with"] # [doc = " something else. This allows us to amortize work if we are printing the"] # [doc = " file path for every match."] # [doc = ""] # [doc = " In the common case, no transformation is needed, which lets us avoid"] # [doc = " the allocation. Typically, only Windows requires a transform, since"] # [doc = " it's fraught to access the raw bytes of a path directly and first need"] # [doc = " to lossily convert to UTF-8. Windows is also typically where the path"] # [doc = " separator replacement is used, e.g., in cygwin environments to use `/`"] # [doc = " instead of `\\`."] # [doc = ""] # [doc = " Users of this type are expected to construct it from a normal `Path`"] # [doc = " found in the standard library. It can then be written to any `io::Write`"] # [doc = " implementation using the `as_bytes` method. This achieves platform"] # [doc = " portability with a small cost: on Windows, paths that are not valid UTF-16"] # [doc = " will not roundtrip correctly."] # [derive (Clone , Debug)] pub (crate) struct PrinterPath < 'a > { # [cfg (not (unix))] path : & 'a Path , bytes : Cow < 'a , [u8] > , hyperlink : OnceCell < Option < HyperlinkPath > > , }
};
}
