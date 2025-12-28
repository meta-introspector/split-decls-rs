macro_rules! deps {
    () => {
        SymbolMap!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl SymbolMap { pub fn new () -> Self { SymbolMap { map : HashMap :: new () , } } pub fn populate_from_cargo_metadata (& mut self , workspace_path : & Path) -> Result < () > { let metadata = MetadataCommand :: new () . current_dir (workspace_path) . exec () . context ("Failed to run cargo metadata") ? ; for package in metadata . packages { let crate_name = package . name ; self . map . insert (crate_name . to_string () , ResolvedDependency { id : crate_name . to_string () , dependency_type : "crate" . to_string () , crate_name : crate_name . to_string () , module_path : crate_name . to_string () , usage_count : 0 , } ,) ; } Ok (()) } pub fn resolve (& self , id : & str) -> Option < ResolvedDependency > { self . map . get (id) . cloned () } pub fn add_declaration (& mut self , id : String , dependency_type : String , crate_name : String , module_path : String ,) { self . map . entry (id . clone ()) . or_insert_with (| | ResolvedDependency { id , dependency_type , crate_name , module_path , usage_count : 0 , }) ; } pub fn resolve_and_increment_usage (& mut self , id : String , dependency_type : String , crate_name : String , module_path : String ,) -> ResolvedDependency { let entry = self . map . entry (id . clone ()) . or_insert_with (| | ResolvedDependency { id , dependency_type , crate_name , module_path , usage_count : 0 , }) ; entry . usage_count += 1 ; entry . clone () } }
    };
}

impl_144!();