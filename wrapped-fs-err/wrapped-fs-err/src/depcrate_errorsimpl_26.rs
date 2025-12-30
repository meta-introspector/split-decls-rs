// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorsimpl_26 {
() => {
// Module: crate::errors
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Display for SourceDestError { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let from = self . from_path . display () ; let to = self . to_path . display () ; match self . kind { SourceDestErrorKind :: Copy => { write ! (formatter , "failed to copy file from {} to {}" , from , to) } SourceDestErrorKind :: HardLink => { write ! (formatter , "failed to hardlink file from {} to {}" , from , to) } SourceDestErrorKind :: Rename => { write ! (formatter , "failed to rename file from {} to {}" , from , to) } SourceDestErrorKind :: SoftLink => { write ! (formatter , "failed to softlink file from {} to {}" , from , to) } # [cfg (unix)] SourceDestErrorKind :: Symlink => { write ! (formatter , "failed to symlink file from {} to {}" , from , to) } # [cfg (windows)] SourceDestErrorKind :: SymlinkFile => { write ! (formatter , "failed to symlink file from {} to {}" , from , to) } # [cfg (windows)] SourceDestErrorKind :: SymlinkDir => { write ! (formatter , "failed to symlink dir from {} to {}" , from , to) } } ? ; # [cfg (not (feature = "expose_original_error"))] write ! (formatter , ": {}" , self . source) ? ; Ok (()) } }
};
}
