macro_rules! deps {
    () => {
        Style!();
        LabelledGraphWithEscStrs!();
    };
}

macro_rules! left_aligned_text {
    () => {
        deps!();
        # [test] fn left_aligned_text () { let labels = AllNodesLabelled (vec ! ["if test {\
       \\l    branch1\
       \\l} else {\
       \\l    branch2\
       \\l}\
       \\lafterward\
       \\l" , "branch1" , "branch2" , "afterward" ,]) ; let mut writer = Vec :: new () ; let g = LabelledGraphWithEscStrs :: new ("syntax_tree" , labels , vec ! [edge (0 , 1 , "then" , Style :: None) , edge (0 , 2 , "else" , Style :: None) , edge (1 , 3 , ";" , Style :: None) , edge (2 , 3 , ";" , Style :: None) ,] ,) ; render (& g , & mut writer) . unwrap () ; let mut r = String :: new () ; Read :: read_to_string (& mut & * writer , & mut r) . unwrap () ; assert_eq ! (r , r#"digraph syntax_tree {
    N0[label="if test {\l    branch1\l} else {\l    branch2\l}\lafterward\l"];
    N1[label="branch1"];
    N2[label="branch2"];
    N3[label="afterward"];
    N0 -> N1[label="then"];
    N0 -> N2[label="else"];
    N1 -> N3[label=";"];
    N2 -> N3[label=";"];
}
"#) ; }
    };
}

left_aligned_text!()