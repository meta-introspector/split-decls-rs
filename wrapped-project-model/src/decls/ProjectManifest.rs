macro_rules! deps {
    () => {
        ManifestPath!();
        ProjectJson!();
    };
}

macro_rules! ProjectManifest {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum ProjectManifest { ProjectJson (ManifestPath) , CargoToml (ManifestPath) , CargoScript (ManifestPath) , }
    };
}

ProjectManifest!();