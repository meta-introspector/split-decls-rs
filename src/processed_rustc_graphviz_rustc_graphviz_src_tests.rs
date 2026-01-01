/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: io ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_USE_0002
/* FP:tests.rs-0004 */ use std :: io :: prelude :: * ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_USE_0003
/* FP:tests.rs-0006 */ use NodeLabels :: * ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_USE_0004
/* FP:tests.rs-0008 */ use super :: LabelText :: { self , EscStr , HtmlStr , LabelStr } ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_USE_0005
/* FP:tests.rs-0010 */ use super :: { Edges , GraphWalk , Id , Labeller , Nodes , Style , render } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_TYPE_0006
/* FP:tests.rs-0012 */ # [doc = " each node is an index in a vector in the graph."] type Node = usize ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_STRUCT_0007
/* FP:tests.rs-0014 */ struct Edge { from : usize , to : usize , label : & 'static str , style : Style , }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0008
/* FP:tests.rs-0016 */ fn edge (from : usize , to : usize , label : & 'static str , style : Style) -> Edge { Edge { from , to , label , style } }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_STRUCT_0009
/* FP:tests.rs-0018 */ struct LabelledGraph { # [doc = " The name for this graph. Used for labeling generated `digraph`."] name : & 'static str , # [doc = " Each node is an index into `node_labels`; these labels are"] # [doc = " used as the label text for each node. (The node *names*,"] # [doc = " which are unique identifiers, are derived from their index"] # [doc = " in this array.)"] # [doc = ""] # [doc = " If a node maps to None here, then just use its name as its"] # [doc = " text."] node_labels : Vec < Option < & 'static str > > , node_styles : Vec < Style > , # [doc = " Each edge relates a from-index to a to-index along with a"] # [doc = " label; `edges` collects them."] edges : Vec < Edge > , }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_STRUCT_0010
/* FP:tests.rs-0020 */ struct LabelledGraphWithEscStrs { graph : LabelledGraph , }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_ENUM_0011
/* FP:tests.rs-0022 */ enum NodeLabels < L > { AllNodesLabelled (Vec < L >) , UnlabelledNodes (usize) , SomeNodesLabelled (Vec < Option < L > >) , }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_TYPE_0012
/* FP:tests.rs-0024 */ type Trivial = NodeLabels < & 'static str > ;
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0013
/* FP:tests.rs-0026 */ impl NodeLabels < & 'static str > { fn to_opt_strs (self) -> Vec < Option < & 'static str > > { match self { UnlabelledNodes (len) => vec ! [None ; len] , AllNodesLabelled (lbls) => lbls . into_iter () . map (Some) . collect () , SomeNodesLabelled (lbls) => lbls , } } fn len (& self) -> usize { match self { & UnlabelledNodes (len) => len , & AllNodesLabelled (ref lbls) => lbls . len () , & SomeNodesLabelled (ref lbls) => lbls . len () , } } }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0014
/* FP:tests.rs-0028 */ impl LabelledGraph { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge > , node_styles : Option < Vec < Style > > ,) -> LabelledGraph { let count = node_labels . len () ; LabelledGraph { name , node_labels : node_labels . to_opt_strs () , edges , node_styles : match node_styles { Some (nodes) => nodes , None => vec ! [Style :: None ; count] , } , } } }
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0015
/* FP:tests.rs-0030 */ impl LabelledGraphWithEscStrs { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge >) -> LabelledGraphWithEscStrs { LabelledGraphWithEscStrs { graph : LabelledGraph :: new (name , node_labels , edges , None) } } }
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0016
/* FP:tests.rs-0032 */ fn id_name < 'a > (n : & Node) -> Id < 'a > { Id :: new (format ! ("N{}" , * n)) . unwrap () }
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0017
/* FP:tests.rs-0034 */ impl < 'a > Labeller < 'a > for LabelledGraph { type Node = Node ; type Edge = & 'a Edge ; fn graph_id (& 'a self) -> Id < 'a > { Id :: new (self . name) . unwrap () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { id_name (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . node_labels [* n] { Some (l) => LabelStr (l . into ()) , None => LabelStr (id_name (n) . name) , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { LabelStr (e . label . into ()) } fn node_style (& 'a self , n : & Node) -> Style { self . node_styles [* n] } fn edge_style (& 'a self , e : & & 'a Edge) -> Style { e . style } }
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0018
/* FP:tests.rs-0036 */ impl < 'a > Labeller < 'a > for LabelledGraphWithEscStrs { type Node = Node ; type Edge = & 'a Edge ; fn graph_id (& 'a self) -> Id < 'a > { self . graph . graph_id () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { self . graph . node_id (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . graph . node_label (n) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { match self . graph . edge_label (e) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } }
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0019
/* FP:tests.rs-0038 */ impl < 'a > GraphWalk < 'a > for LabelledGraph { type Node = Node ; type Edge = & 'a Edge ; fn nodes (& 'a self) -> Nodes < 'a , Node > { (0 .. self . node_labels . len ()) . collect () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . edges . iter () . collect () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } }
/* FP:tests.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_IMPL_0020
/* FP:tests.rs-0040 */ impl < 'a > GraphWalk < 'a > for LabelledGraphWithEscStrs { type Node = Node ; type Edge = & 'a Edge ; fn nodes (& 'a self) -> Nodes < 'a , Node > { self . graph . nodes () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . graph . edges () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } }
/* FP:tests.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0021
/* FP:tests.rs-0042 */ fn test_input (g : LabelledGraph) -> io :: Result < String > { let mut writer = Vec :: new () ; render (& g , & mut writer) . unwrap () ; let mut s = String :: new () ; Read :: read_to_string (& mut & * writer , & mut s) ? ; Ok (s) }
/* FP:tests.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0022
/* FP:tests.rs-0044 */ # [test] fn empty_graph () { let labels : Trivial = UnlabelledNodes (0) ; let r = test_input (LabelledGraph :: new ("empty_graph" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph empty_graph {
/* FP:tests.rs-0045 */ }
/* FP:tests.rs-0046 */ "#) ; }
/* FP:tests.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0023
/* FP:tests.rs-0048 */ # [test] fn single_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
/* FP:tests.rs-0049 */     N0[label="N0"];
/* FP:tests.rs-0050 */ }
/* FP:tests.rs-0051 */ "#) ; }
/* FP:tests.rs-0052 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0024
/* FP:tests.rs-0053 */ # [test] fn single_node_with_style () { let labels : Trivial = UnlabelledNodes (1) ; let styles = Some (vec ! [Style :: Dashed]) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , styles)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
/* FP:tests.rs-0054 */     N0[label="N0"][style="dashed"];
/* FP:tests.rs-0055 */ }
/* FP:tests.rs-0056 */ "#) ; }
/* FP:tests.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0025
/* FP:tests.rs-0058 */ # [test] fn single_edge () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: None)] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
/* FP:tests.rs-0059 */     N0[label="N0"];
/* FP:tests.rs-0060 */     N1[label="N1"];
/* FP:tests.rs-0061 */     N0 -> N1[label="E"];
/* FP:tests.rs-0062 */ }
/* FP:tests.rs-0063 */ "#) ; }
/* FP:tests.rs-0064 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0026
/* FP:tests.rs-0065 */ # [test] fn single_edge_with_style () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: Bold)] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
/* FP:tests.rs-0066 */     N0[label="N0"];
/* FP:tests.rs-0067 */     N1[label="N1"];
/* FP:tests.rs-0068 */     N0 -> N1[label="E"][style="bold"];
/* FP:tests.rs-0069 */ }
/* FP:tests.rs-0070 */ "#) ; }
/* FP:tests.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0027
/* FP:tests.rs-0072 */ # [test] fn test_some_labelled () { let labels : Trivial = SomeNodesLabelled (vec ! [Some ("A") , None]) ; let styles = Some (vec ! [Style :: None , Style :: Dotted]) ; let result = test_input (LabelledGraph :: new ("test_some_labelled" , labels , vec ! [edge (0 , 1 , "A-1" , Style :: None)] , styles ,)) ; assert_eq ! (result . unwrap () , r#"digraph test_some_labelled {
/* FP:tests.rs-0073 */     N0[label="A"];
/* FP:tests.rs-0074 */     N1[label="N1"][style="dotted"];
/* FP:tests.rs-0075 */     N0 -> N1[label="A-1"];
/* FP:tests.rs-0076 */ }
/* FP:tests.rs-0077 */ "#) ; }
/* FP:tests.rs-0078 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0028
/* FP:tests.rs-0079 */ # [test] fn single_cyclic_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_cyclic_node" , labels , vec ! [edge (0 , 0 , "E" , Style :: None)] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph single_cyclic_node {
/* FP:tests.rs-0080 */     N0[label="N0"];
/* FP:tests.rs-0081 */     N0 -> N0[label="E"];
/* FP:tests.rs-0082 */ }
/* FP:tests.rs-0083 */ "#) ; }
/* FP:tests.rs-0084 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0029
/* FP:tests.rs-0085 */ # [test] fn hasse_diagram () { let labels = AllNodesLabelled (vec ! ["{x,y}" , "{x}" , "{y}" , "{}"]) ; let r = test_input (LabelledGraph :: new ("hasse_diagram" , labels , vec ! [edge (0 , 1 , "" , Style :: None) , edge (0 , 2 , "" , Style :: None) , edge (1 , 3 , "" , Style :: None) , edge (2 , 3 , "" , Style :: None) ,] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph hasse_diagram {
/* FP:tests.rs-0086 */     N0[label="{x,y}"];
/* FP:tests.rs-0087 */     N1[label="{x}"];
/* FP:tests.rs-0088 */     N2[label="{y}"];
/* FP:tests.rs-0089 */     N3[label="{}"];
/* FP:tests.rs-0090 */     N0 -> N1[label=""];
/* FP:tests.rs-0091 */     N0 -> N2[label=""];
/* FP:tests.rs-0092 */     N1 -> N3[label=""];
/* FP:tests.rs-0093 */     N2 -> N3[label=""];
/* FP:tests.rs-0094 */ }
/* FP:tests.rs-0095 */ "#) ; }
/* FP:tests.rs-0096 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0030
/* FP:tests.rs-0097 */ # [test] fn left_aligned_text () { let labels = AllNodesLabelled (vec ! ["if test {\
/* FP:tests.rs-0098 */        \\l    branch1\
/* FP:tests.rs-0099 */        \\l} else {\
/* FP:tests.rs-0100 */        \\l    branch2\
/* FP:tests.rs-0101 */        \\l}\
/* FP:tests.rs-0102 */        \\lafterward\
/* FP:tests.rs-0103 */        \\l" , "branch1" , "branch2" , "afterward" ,]) ; let mut writer = Vec :: new () ; let g = LabelledGraphWithEscStrs :: new ("syntax_tree" , labels , vec ! [edge (0 , 1 , "then" , Style :: None) , edge (0 , 2 , "else" , Style :: None) , edge (1 , 3 , ";" , Style :: None) , edge (2 , 3 , ";" , Style :: None) ,] ,) ; render (& g , & mut writer) . unwrap () ; let mut r = String :: new () ; Read :: read_to_string (& mut & * writer , & mut r) . unwrap () ; assert_eq ! (r , r#"digraph syntax_tree {
/* FP:tests.rs-0104 */     N0[label="if test {\l    branch1\l} else {\l    branch2\l}\lafterward\l"];
/* FP:tests.rs-0105 */     N1[label="branch1"];
/* FP:tests.rs-0106 */     N2[label="branch2"];
/* FP:tests.rs-0107 */     N3[label="afterward"];
/* FP:tests.rs-0108 */     N0 -> N1[label="then"];
/* FP:tests.rs-0109 */     N0 -> N2[label="else"];
/* FP:tests.rs-0110 */     N1 -> N3[label=";"];
/* FP:tests.rs-0111 */     N2 -> N3[label=";"];
/* FP:tests.rs-0112 */ }
/* FP:tests.rs-0113 */ "#) ; }
/* FP:tests.rs-0114 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0031
/* FP:tests.rs-0115 */ # [test] fn simple_id_construction () { let id1 = Id :: new ("hello") ; match id1 { Ok (_) => { } Err (..) => panic ! ("'hello' is not a valid value for id anymore") , } }
/* FP:tests.rs-0116 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_graphviz_src_tests_FN_0032
/* FP:tests.rs-0117 */ # [test] fn badly_formatted_id () { let id2 = Id :: new ("Weird { struct : ure } !!!") ; match id2 { Ok (_) => panic ! ("graphviz id suddenly allows spaces, brackets and stuff") , Err (..) => { } } }