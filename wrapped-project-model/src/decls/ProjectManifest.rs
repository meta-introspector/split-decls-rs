macro_rules! ProjectManifest {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum ProjectManifest { ProjectJson (ManifestPath) , CargoToml (ManifestPath) , CargoScript (ManifestPath) , }
    };
}

ProjectManifest!()