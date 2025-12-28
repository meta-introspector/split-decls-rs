macro_rules! deps {
    () => {
        Config!();
        NodeRef!();
        Graph!();
        RankDir!();
        Escaper!();
        Dot!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use alloc :: { format , string :: String } ; use core :: fmt :: Write ; use super :: { Config , Dot , Escaper , RankDir } ; use crate :: prelude :: Graph ; use crate :: visit :: NodeRef ; # [test] fn test_escape () { let mut buff = String :: new () ; { let mut e = Escaper (& mut buff) ; let _ = e . write_str ("\" \\ \n") ; } assert_eq ! (buff , "\\\" \\\\ \\l") ; } fn simple_graph () -> Graph < & 'static str , & 'static str > { let mut graph = Graph :: < & str , & str > :: new () ; let a = graph . add_node ("A") ; let b = graph . add_node ("B") ; graph . add_edge (a , b , "edge_label") ; graph } # [test] fn test_nodeindexlable_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: NodeIndexLabel])) ; assert_eq ! (dot , "digraph {\n    0 [ label = \"0\" ]\n    1 [ label = \"1\" ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_edgeindexlable_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: EdgeIndexLabel])) ; assert_eq ! (dot , "digraph {\n    0 [ label = \"\\\"A\\\"\" ]\n    1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ label = \"0\" ]\n}\n") ; } # [test] fn test_edgenolable_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: EdgeNoLabel])) ; assert_eq ! (dot , "digraph {\n    0 [ label = \"\\\"A\\\"\" ]\n    1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ ]\n}\n") ; } # [test] fn test_nodenolable_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: NodeNoLabel])) ; assert_eq ! (dot , "digraph {\n    0 [ ]\n    1 [ ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_rankdir_bt_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: RankDir (RankDir :: TB)])) ; assert_eq ! (dot , "digraph {\n    rankdir=\"TB\"\n    0 [ label = \"\\\"A\\\"\" ]\n    \
            1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_rankdir_tb_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: RankDir (RankDir :: BT)])) ; assert_eq ! (dot , "digraph {\n    rankdir=\"BT\"\n    0 [ label = \"\\\"A\\\"\" ]\n    \
            1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_rankdir_lr_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: RankDir (RankDir :: LR)])) ; assert_eq ! (dot , "digraph {\n    rankdir=\"LR\"\n    0 [ label = \"\\\"A\\\"\" ]\n    \
            1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_rankdir_rl_option () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_config (& graph , & [Config :: RankDir (RankDir :: RL)])) ; assert_eq ! (dot , "digraph {\n    rankdir=\"RL\"\n    0 [ label = \"\\\"A\\\"\" ]\n    \
            1 [ label = \"\\\"B\\\"\" ]\n    0 -> 1 [ label = \"\\\"edge_label\\\"\" ]\n}\n") ; } # [test] fn test_with_attr_getters () { let graph = simple_graph () ; let dot = format ! ("{:?}" , Dot :: with_attr_getters (& graph , & [Config :: NodeNoLabel , Config :: EdgeNoLabel] , &| _ , er | format ! ("label = \"{}\"" , er . weight () . to_uppercase ()) , &| _ , nr | format ! ("label = \"{}\"" , nr . weight () . to_lowercase ()) ,) ,) ; assert_eq ! (dot , "digraph {\n    0 [ label = \"a\"]\n    1 [ label = \"b\"]\n    0 -> 1 [ label = \"EDGE_LABEL\"]\n}\n") ; } }
    };
}

test!()