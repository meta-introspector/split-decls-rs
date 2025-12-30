// Generated macro for check (function)
macro_rules! Depcrate_cargo_feature_namecheck {
() => {
// Module: crate::cargo::feature_name
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , metadata : & Metadata) { for package in & metadata . packages { let mut features : Vec < & String > = package . features . keys () . collect () ; features . sort () ; for feature in features { let prefix_opt = { let i = PREFIXES . partition_point (| prefix | prefix < & feature . as_str ()) ; if i > 0 && feature . starts_with (PREFIXES [i - 1]) { Some (PREFIXES [i - 1]) } else { None } } ; if let Some (prefix) = prefix_opt { lint (cx , feature , prefix , true) ; } let suffix_opt : Option < & str > = { let i = SUFFIXES . partition_point (| suffix | { suffix . bytes () . rev () . cmp (feature . bytes () . rev ()) == std :: cmp :: Ordering :: Less }) ; if i > 0 && feature . ends_with (SUFFIXES [i - 1]) { Some (SUFFIXES [i - 1]) } else { None } } ; if let Some (suffix) = suffix_opt { lint (cx , feature , suffix , false) ; } } } }
};
}
