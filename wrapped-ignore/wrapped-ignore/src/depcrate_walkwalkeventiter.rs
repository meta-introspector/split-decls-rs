// Generated macro for WalkEventIter (struct)
macro_rules! Depcrate_walkWalkEventIter {
() => {
// Module: crate::walk
// Provides: {"WalkEventIter"}
// Dependencies: {}
# [doc = " WalkEventIter transforms a WalkDir iterator into an iterator that more"] # [doc = " accurately describes the directory tree. Namely, it emits events that are"] # [doc = " one of three types: directory, file or \"exit.\" An \"exit\" event means that"] # [doc = " the entire contents of a directory have been enumerated."] struct WalkEventIter { depth : usize , it : walkdir :: IntoIter , next : Option < Result < walkdir :: DirEntry , walkdir :: Error > > , }
};
}
