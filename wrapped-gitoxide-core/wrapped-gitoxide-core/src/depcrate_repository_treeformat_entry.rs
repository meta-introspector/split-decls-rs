// Generated macro for format_entry (function)
macro_rules! Depcrate_repository_treeformat_entry {
() => {
// Module: crate::repository::tree
// Provides: {"format_entry"}
// Dependencies: {}
fn format_entry (mut out : impl io :: Write , entry : & gix :: objs :: tree :: EntryRef < '_ > , filename : & gix :: bstr :: BStr , size : Option < u64 > ,) -> std :: io :: Result < () > { use gix :: objs :: tree :: EntryKind :: * ; write ! (out , "{} {}{} " , match entry . mode . kind () { Tree => "TREE" , Blob => "BLOB" , BlobExecutable => " EXE" , Link => "LINK" , Commit => "SUBM" , } , entry . oid , size . map_or_else (|| "" . into () , | s | Cow :: Owned (format ! (" {s}")))) ? ; out . write_all (filename) ? ; out . write_all (b"\n") }
};
}
