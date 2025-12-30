// Generated macro for Work (struct)
macro_rules! Depcrate_walkWork {
() => {
// Module: crate::walk
// Provides: {"Work"}
// Dependencies: {}
# [doc = " A unit of work for each worker to process."] # [doc = ""] # [doc = " Each unit of work corresponds to a directory that should be descended"] # [doc = " into."] struct Work { # [doc = " The directory entry."] dent : DirEntry , # [doc = " Any ignore matchers that have been built for this directory's parents."] ignore : Ignore , # [doc = " The root device number. When present, only files with the same device"] # [doc = " number should be considered."] root_device : Option < u64 > , }
};
}
