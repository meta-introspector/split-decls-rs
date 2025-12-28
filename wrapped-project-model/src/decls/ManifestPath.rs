macro_rules! ManifestPath {
    () => {
        # [doc = " More or less [`AbsPathBuf`] with non-None parent."] # [doc = ""] # [doc = " We use it to store path to Cargo.toml, as we frequently use the parent dir"] # [doc = " as a working directory to spawn various commands, and its nice to not have"] # [doc = " to `.unwrap()` everywhere."] # [doc = ""] # [doc = " This could have been named `AbsNonRootPathBuf`, as we don't enforce that"] # [doc = " this stores manifest files in particular, but we only use this for manifests"] # [doc = " at the moment in practice."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct ManifestPath { file : AbsPathBuf , }
    };
}

ManifestPath!();