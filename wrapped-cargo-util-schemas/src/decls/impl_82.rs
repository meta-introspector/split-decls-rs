macro_rules! deps {
    () => {
        TomlPackage!();
        TomlLints!();
        Result!();
        InheritableDependency!();
        TomlManifest!();
        UnresolvedError!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl TomlManifest { pub fn requires_package (& self) -> impl Iterator < Item = & 'static str > { [self . badges . as_ref () . map (| _ | "badges") , self . features . as_ref () . map (| _ | "features") , self . lib . as_ref () . map (| _ | "lib") , self . bin . as_ref () . map (| _ | "bin") , self . example . as_ref () . map (| _ | "example") , self . test . as_ref () . map (| _ | "test") , self . bench . as_ref () . map (| _ | "bench") , self . dependencies . as_ref () . map (| _ | "dependencies") , self . dev_dependencies () . as_ref () . map (| _ | "dev-dependencies") , self . build_dependencies () . as_ref () . map (| _ | "build-dependencies") , self . target . as_ref () . map (| _ | "target") , self . lints . as_ref () . map (| _ | "lints") , self . hints . as_ref () . map (| _ | "hints") ,] . into_iter () . flatten () } pub fn has_profiles (& self) -> bool { self . profile . is_some () } pub fn package (& self) -> Option < & Box < TomlPackage > > { self . package . as_ref () . or (self . project . as_ref ()) } pub fn dev_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . dev_dependencies . as_ref () . or (self . dev_dependencies2 . as_ref ()) } pub fn build_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . build_dependencies . as_ref () . or (self . build_dependencies2 . as_ref ()) } pub fn features (& self) -> Option < & BTreeMap < FeatureName , Vec < String > > > { self . features . as_ref () } pub fn normalized_lints (& self) -> Result < Option < & TomlLints > , UnresolvedError > { self . lints . as_ref () . map (| l | l . normalized ()) . transpose () } }
    };
}

impl_82!()