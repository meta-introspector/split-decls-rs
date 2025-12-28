macro_rules! deps {
    () => {
        RefLogMessage!();
        Prepare!();
        WritePackedRefs!();
        DryRun!();
    };
}

macro_rules! impl_965 {
    () => {
        deps!();
        # [doc = " Builder"] impl < T > Prepare < '_ , '_ , T > where T : Transport , { # [doc = " If dry run is enabled, no change to the repository will be made."] # [doc = ""] # [doc = " This works by not actually fetching the pack after negotiating it, nor will refs be updated."] pub fn with_dry_run (mut self , enabled : bool) -> Self { self . dry_run = if enabled { DryRun :: Yes } else { DryRun :: No } ; self } # [doc = " If enabled, don't write ref updates to loose refs, but put them exclusively to packed-refs."] # [doc = ""] # [doc = " This improves performance and allows case-sensitive filesystems to deal with ref names that would otherwise"] # [doc = " collide."] pub fn with_write_packed_refs_only (mut self , enabled : bool) -> Self { self . write_packed_refs = if enabled { WritePackedRefs :: Only } else { WritePackedRefs :: Never } ; self } # [doc = " Set the reflog message to use when updating refs after fetching a pack."] pub fn with_reflog_message (mut self , reflog_message : RefLogMessage) -> Self { self . reflog_message = reflog_message . into () ; self } # [doc = " Define what to do when the current repository is a shallow clone."] # [doc = ""] # [doc = " *Has no effect if the current repository is not as shallow clone.*"] pub fn with_shallow (mut self , shallow : remote :: fetch :: Shallow) -> Self { self . shallow = shallow ; self } }
    };
}

impl_965!();