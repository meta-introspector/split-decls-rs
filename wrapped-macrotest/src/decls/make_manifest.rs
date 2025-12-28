macro_rules! deps {
    () => {
        Project!();
        WorkspacePackage!();
        Package!();
        ExpandedTest!();
        Name!();
        Bin!();
        Manifest!();
        Workspace!();
        Result!();
        Dependency!();
    };
}

macro_rules! make_manifest {
    () => {
        deps!();
        fn make_manifest (crate_name : String , project : & Project , tests : & [ExpandedTest] ,) -> Result < Manifest > { let source_manifest = dependencies :: get_manifest (& project . source_dir) ; let workspace_manifest = dependencies :: get_workspace_manifest (& project . workspace) ; let features = source_manifest . features . iter () . map (| (feature , source_deps) | { let enable = format ! ("{}/{}" , crate_name , feature) ; let mut deps = vec ! [enable] ; deps . extend (source_deps . iter () . filter (| dep | dep . starts_with ("dep:")) . cloned () ,) ; (feature . clone () , deps) }) . collect () ; let mut manifest = Manifest { cargo_features : source_manifest . cargo_features . clone () , package : Package { name : project . name . clone () , version : "0.0.0" . to_owned () , edition : source_manifest . package . edition , publish : false , } , features , dependencies : std :: collections :: BTreeMap :: new () , bins : Vec :: new () , workspace : Some (Workspace { package : crate :: manifest :: WorkspacePackage { edition : workspace_manifest . workspace . package . edition , } , dependencies : workspace_manifest . workspace . dependencies , }) , patch : workspace_manifest . patch , replace : workspace_manifest . replace , } ; manifest . dependencies . extend (source_manifest . dependencies) ; manifest . dependencies . extend (source_manifest . dev_dependencies) ; manifest . dependencies . insert (crate_name , Dependency { version : None , path : Some (project . source_dir . clone ()) , default_features : false , features : Vec :: new () , workspace : false , rest : std :: collections :: BTreeMap :: new () , } ,) ; manifest . bins . push (Bin { name : Name (project . name . to_owned ()) , path : Path :: new ("main.rs") . to_owned () , }) ; for expanded in tests { if expanded . error . is_none () { manifest . bins . push (Bin { name : expanded . name . clone () , path : project . source_dir . join (& expanded . test) , }) ; } } Ok (manifest) }
    };
}

make_manifest!()