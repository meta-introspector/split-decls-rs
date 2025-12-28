macro_rules! deps {
    () => {
        ManifestPath!();
        TargetKind!();
    };
}

macro_rules! Build {
    () => {
        deps!();
        # [doc = " Additional, build-specific data about a crate."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Build { # [doc = " The name associated with this crate."] # [doc = ""] # [doc = " This is determined by the build system that produced"] # [doc = " the `rust-project.json` in question. For instance, if buck were used,"] # [doc = " the label might be something like `//ide/rust/rust-analyzer:rust-analyzer`."] # [doc = ""] # [doc = " Do not attempt to parse the contents of this string; it is a build system-specific"] # [doc = " identifier similar to [`Crate::display_name`]."] pub label : String , # [doc = " Path corresponding to the build system-specific file defining the crate."] # [doc = ""] # [doc = " It is roughly analogous to [`ManifestPath`], but it should *not* be used with"] # [doc = " [`crate::ProjectManifest::from_manifest_file`], as the build file may not be"] # [doc = " be in the `rust-project.json`."] pub build_file : Utf8PathBuf , # [doc = " The kind of target."] # [doc = ""] # [doc = " Examples (non-exhaustively) include [`TargetKind::Bin`], [`TargetKind::Lib`],"] # [doc = " and [`TargetKind::Test`]. This information is used to determine what sort"] # [doc = " of runnable codelens to provide, if any."] pub target_kind : TargetKind , }
    };
}

Build!()