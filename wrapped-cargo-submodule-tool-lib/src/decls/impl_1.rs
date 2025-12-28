macro_rules! deps {
    () => {
        RealDepGraphProcessor!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl DepGraphProcessor for RealDepGraphProcessor { # [cfg (feature = "nix_generation")] fn process_dep_graph (& self , dot_content : & str ,) -> Result < HashMap < String , MergedCrateInfo > > { lazy_static ! { static ref NODE_RE : Regex = Regex :: new (r#"^  "([^"]+)" \[label="([^"]+)"\];$"#) . unwrap () ; static ref EDGE_RE : Regex = Regex :: new (r#"^  "([^"]+)" -> "([^"]+)"(?: \[label="([^"]+)"\])?;$"#) . unwrap () ; static ref VERSION_RE : Regex = Regex :: new (r#" v\d+\.\d+\.\d+(?:-\S+)?"#) . unwrap () ; } let mut nodes = HashMap :: new () ; let mut edges = Vec :: new () ; for line in dot_content . lines () { if let Some (captures) = NODE_RE . captures (line) { let mut node_name = captures [1] . to_string () ; let label = captures [2] . to_string () ; node_name = VERSION_RE . replace_all (& node_name , "") . trim () . to_string () ; nodes . insert (node_name , MergedCrateInfo { layer : - 1 , usage_count : 0 , } ,) ; } else if let Some (captures) = EDGE_RE . captures (line) { let mut from_node = captures [1] . to_string () ; let mut to_node = captures [2] . to_string () ; from_node = VERSION_RE . replace_all (& from_node , "") . trim () . to_string () ; to_node = VERSION_RE . replace_all (& to_node , "") . trim () . to_string () ; edges . push ((from_node , to_node)) ; } } Ok (nodes) } # [cfg (not (feature = "nix_generation"))] fn process_dep_graph (& self , _dot_content : & str ,) -> Result < HashMap < String , MergedCrateInfo > > { anyhow :: bail ! ("`DepGraphProcessor` requires the `nix_generation` feature to be enabled.") ; } }
    };
}

impl_1!();