macro_rules! crate_graph_dedup {
    () => {
        # [test] fn crate_graph_dedup () { let mut file_map = Default :: default () ; let ripgrep_workspace = load_workspace_from_metadata ("ripgrep-metadata.json") ; let (mut crate_graph , _proc_macros) = to_crate_graph (ripgrep_workspace , & mut file_map) ; assert_eq ! (crate_graph . iter () . count () , 71) ; let regex_workspace = load_workspace_from_metadata ("regex-metadata.json") ; let (regex_crate_graph , mut regex_proc_macros) = to_crate_graph (regex_workspace , & mut file_map) ; assert_eq ! (regex_crate_graph . iter () . count () , 50) ; crate_graph . extend (regex_crate_graph , & mut regex_proc_macros) ; assert_eq ! (crate_graph . iter () . count () , 108) ; }
    };
}

crate_graph_dedup!()