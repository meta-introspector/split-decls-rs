// Generated macro for write (function)
macro_rules! Depcrate_fswrite {
() => {
// Module: crate::fs
// Provides: {"write"}
// Dependencies: {}
# [doc = " Write a slice as the entire contents of a file."] # [doc = ""] # [doc = " This function is identical to [`std::fs::write`] but uses [`Close`]"] # [doc = " to drop the [`File`](std::fs::File) created from path, returning any"] # [doc = " errors encountered when doing so."] pub fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C ,) -> Result < () > { fn inner (path : & Path , contents : & [u8]) -> Result < () > { let mut f = File :: create (path) ? ; f . write_all (contents) ? ; f . close () } inner (path . as_ref () , contents . as_ref ()) }
};
}
