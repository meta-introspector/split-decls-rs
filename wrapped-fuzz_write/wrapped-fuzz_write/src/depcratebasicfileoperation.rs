// Generated macro for BasicFileOperation (enum)
macro_rules! DepcrateBasicFileOperation {
() => {
// Module: crate
// Provides: {"BasicFileOperation"}
// Dependencies: {}
# [derive (Arbitrary , Clone)] pub enum BasicFileOperation < 'k > { WriteNormalFile { contents : Box < [Box < [u8] >] > , options : FullFileOptions < 'k > , } , WriteDirectory (FullFileOptions < 'k >) , WriteSymlinkWithTarget { target : PathBuf , options : FullFileOptions < 'k > , } , ShallowCopy (Box < FileOperation < 'k > >) , DeepCopy (Box < FileOperation < 'k > >) , MergeWithOtherFile { initial_junk : Box < [u8] > , operations : Box < [(FileOperation < 'k > , bool)] > , } , SetArchiveComment (Box < [u8] >) , }
};
}
