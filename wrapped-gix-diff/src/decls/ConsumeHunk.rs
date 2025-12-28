macro_rules! deps {
    () => {
        DiffLineKind!();
        HunkHeader!();
    };
}

macro_rules! ConsumeHunk {
    () => {
        deps!();
        # [doc = " A utility trait for use in [`UnifiedDiff`](super::UnifiedDiff)."] pub trait ConsumeHunk { # [doc = " The item this instance produces after consuming all hunks."] type Out ; # [doc = " Consume a single hunk which is represented by its `lines`, each of which with a `DiffLineKind` value"] # [doc = " to know if it's added, removed or context."] # [doc = " The `header` specifies hunk offsets, which positions the `lines` in the old and new file respectively."] # [doc = ""] # [doc = " Note that the [`UnifiedDiff`](super::UnifiedDiff) sink will wrap its output in an [`std::io::Result`]."] # [doc = " After this method returned its first error, it will not be called anymore."] fn consume_hunk (& mut self , header : HunkHeader , lines : & [(DiffLineKind , & [u8])]) -> std :: io :: Result < () > ; # [doc = " Called after the last hunk is consumed to produce an output."] fn finish (self) -> Self :: Out ; }
    };
}

ConsumeHunk!();