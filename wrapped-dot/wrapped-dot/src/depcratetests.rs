// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use self :: NodeLabels :: * ; use super :: LabelText :: { self , EscStr , HtmlStr , LabelStr } ; use super :: { render , Edges , GraphWalk , Id , Kind , Labeller , Nodes , RankDir , Style } ; use super :: { Arrow , ArrowShape , Side } ; use std :: io ; use std :: io :: prelude :: * ; # [doc = " each node is an index in a vector in the graph."] type Node = usize ; struct Edge { from : usize , to : usize , label : & 'static str , style : Style , start_arrow : Arrow , end_arrow : Arrow , color : Option < & 'static str > , } fn edge (from : usize , to : usize , label : & 'static str , style : Style , color : Option < & 'static str > ,) -> Edge { Edge { from , to , label , style , start_arrow : Arrow :: default () , end_arrow : Arrow :: default () , color , } } fn edge_with_arrows (from : usize , to : usize , label : & 'static str , style : Style , start_arrow : Arrow , end_arrow : Arrow , color : Option < & 'static str > ,) -> Edge { Edge { from , to , label , style , start_arrow , end_arrow , color , } } struct LabelledGraph { # [doc = " The name for this graph. Used for labelling generated `digraph`."] name : & 'static str , # [doc = " Each node is an index into `node_labels`; these labels are"] # [doc = " used as the label text for each node. (The node *names*,"] # [doc = " which are unique identifiers, are derived from their index"] # [doc = " in this array.)"] # [doc = ""] # [doc = " If a node maps to None here, then just use its name as its"] # [doc = " text."] node_labels : Vec < Option < & 'static str > > , node_styles : Vec < Style > , # [doc = " Each edge relates a from-index to a to-index along with a"] # [doc = " label; `edges` collects them."] edges : Vec < Edge > , } struct LabelledGraphWithEscStrs { graph : LabelledGraph , } enum NodeLabels < L > { AllNodesLabelled (Vec < L >) , UnlabelledNodes (usize) , SomeNodesLabelled (Vec < Option < L > >) , } type Trivial = NodeLabels < & 'static str > ; impl NodeLabels < & 'static str > { fn into_opt_strs (self) -> Vec < Option < & 'static str > > { match self { UnlabelledNodes (len) => vec ! [None ; len] , AllNodesLabelled (lbls) => lbls . into_iter () . map (Some) . collect () , SomeNodesLabelled (lbls) => lbls . into_iter () . collect () , } } fn len (& self) -> usize { match self { & UnlabelledNodes (len) => len , & AllNodesLabelled (ref lbls) => lbls . len () , & SomeNodesLabelled (ref lbls) => lbls . len () , } } } impl LabelledGraph { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge > , node_styles : Option < Vec < Style > > ,) -> LabelledGraph { let count = node_labels . len () ; LabelledGraph { name , node_labels : node_labels . into_opt_strs () , edges , node_styles : match node_styles { Some (nodes) => nodes , None => vec ! [Style :: None ; count] , } , } } } impl LabelledGraphWithEscStrs { fn new (name : & 'static str , node_labels : Trivial , edges : Vec < Edge > ,) -> LabelledGraphWithEscStrs { LabelledGraphWithEscStrs { graph : LabelledGraph :: new (name , node_labels , edges , None) , } } } fn id_name < 'a > (n : & Node) -> Id < 'a > { Id :: new (format ! ("N{}" , * n)) . unwrap () } impl < 'a > Labeller < 'a , Node , & 'a Edge > for LabelledGraph { fn graph_id (& 'a self) -> Id < 'a > { Id :: new (self . name) . unwrap () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { id_name (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . node_labels [* n] { Some (l) => LabelStr (l . into ()) , None => LabelStr (id_name (n) . name ()) , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { LabelStr (e . label . into ()) } fn node_style (& 'a self , n : & Node) -> Style { self . node_styles [* n] } fn edge_style (& 'a self , e : & & 'a Edge) -> Style { e . style } fn edge_color (& 'a self , e : & & 'a Edge) -> Option < LabelText < 'a > > { e . color . map (| l | LabelStr ((* l) . into ())) } fn edge_end_arrow (& 'a self , e : & & 'a Edge) -> Arrow { e . end_arrow . clone () } fn edge_start_arrow (& 'a self , e : & & 'a Edge) -> Arrow { e . start_arrow . clone () } } impl < 'a > Labeller < 'a , Node , & 'a Edge > for LabelledGraphWithEscStrs { fn graph_id (& 'a self) -> Id < 'a > { self . graph . graph_id () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { self . graph . node_id (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . graph . node_label (n) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } fn node_color (& 'a self , n : & Node) -> Option < LabelText < 'a > > { match self . graph . node_color (n) { Some (LabelStr (s)) | Some (EscStr (s)) | Some (HtmlStr (s)) => Some (EscStr (s)) , None => None , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { match self . graph . edge_label (e) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } fn edge_color (& 'a self , e : & & 'a Edge) -> Option < LabelText < 'a > > { match self . graph . edge_color (e) { Some (LabelStr (s)) | Some (EscStr (s)) | Some (HtmlStr (s)) => Some (EscStr (s)) , None => None , } } } impl < 'a > GraphWalk < 'a , Node , & 'a Edge > for LabelledGraph { fn nodes (& 'a self) -> Nodes < 'a , Node > { (0 .. self . node_labels . len ()) . collect () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . edges . iter () . collect () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } } impl < 'a > GraphWalk < 'a , Node , & 'a Edge > for LabelledGraphWithEscStrs { fn nodes (& 'a self) -> Nodes < 'a , Node > { self . graph . nodes () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . graph . edges () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } } fn test_input (g : LabelledGraph) -> io :: Result < String > { let mut writer = Vec :: new () ; render (& g , & mut writer) ? ; let mut s = String :: new () ; Read :: read_to_string (& mut & * writer , & mut s) ? ; Ok (s) } # [test] fn empty_graph () { let labels : Trivial = UnlabelledNodes (0) ; let r = test_input (LabelledGraph :: new ("empty_graph" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph empty_graph {
}
"#) ; } # [test] fn single_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"];
}
"#) ; } # [test] fn single_node_with_style () { let labels : Trivial = UnlabelledNodes (1) ; let styles = Some (vec ! [Style :: Dashed]) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , styles)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#) ; } # [test] fn single_edge () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: None , None)] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"];
}
"#) ; } # [test] fn single_edge_with_style () { let labels : Trivial = UnlabelledNodes (2) ; let result = test_input (LabelledGraph :: new ("single_edge" , labels , vec ! [edge (0 , 1 , "E" , Style :: Bold , Some ("red"))] , None ,)) ; assert_eq ! (result . unwrap () , r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"][style="bold"][color="red"];
}
"#) ; } # [test] fn test_some_labelled () { let labels : Trivial = SomeNodesLabelled (vec ! [Some ("A") , None]) ; let styles = Some (vec ! [Style :: None , Style :: Dotted]) ; let result = test_input (LabelledGraph :: new ("test_some_labelled" , labels , vec ! [edge (0 , 1 , "A-1" , Style :: None , None)] , styles ,)) ; assert_eq ! (result . unwrap () , r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#) ; } # [test] fn single_cyclic_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_cyclic_node" , labels , vec ! [edge (0 , 0 , "E" , Style :: None , None)] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#) ; } # [test] fn hasse_diagram () { let labels = AllNodesLabelled (vec ! ["{x,y}" , "{x}" , "{y}" , "{}"]) ; let r = test_input (LabelledGraph :: new ("hasse_diagram" , labels , vec ! [edge (0 , 1 , "" , Style :: None , Some ("green")) , edge (0 , 2 , "" , Style :: None , Some ("blue")) , edge (1 , 3 , "" , Style :: None , Some ("red")) , edge (2 , 3 , "" , Style :: None , Some ("black")) ,] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph hasse_diagram {
    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""][color="green"];
    N0 -> N2[label=""][color="blue"];
    N1 -> N3[label=""][color="red"];
    N2 -> N3[label=""][color="black"];
}
"#) ; } # [test] fn utf8_diagram () { let labels = AllNodesLabelled (vec ! ("Λ" , "ι")) ; let r = test_input (LabelledGraph :: new ("utf8_diagram" , labels , vec ! [edge (0 , 1 , "☕" , Style :: None , None)] , None)) ; assert_eq ! (r . unwrap () , r#"digraph utf8_diagram {
    N0[label="Λ"];
    N1[label="ι"];
    N0 -> N1[label="☕"];
}
"#) ; } # [test] fn left_aligned_text () { let labels = AllNodesLabelled (vec ! ["if test {\
           \\l    branch1\
           \\l} else {\
           \\l    branch2\
           \\l}\
           \\lafterward\
           \\l" , "branch1" , "branch2" , "afterward" ,]) ; let mut writer = Vec :: new () ; let g = LabelledGraphWithEscStrs :: new ("syntax_tree" , labels , vec ! [edge (0 , 1 , "then" , Style :: None , None) , edge (0 , 2 , "else" , Style :: None , None) , edge (1 , 3 , ";" , Style :: None , None) , edge (2 , 3 , ";" , Style :: None , None) ,] ,) ; render (& g , & mut writer) . unwrap () ; let mut r = String :: new () ; Read :: read_to_string (& mut & * writer , & mut r) . unwrap () ; assert_eq ! (r , r#"digraph syntax_tree {
    N0[label="if test {\l    branch1\l} else {\l    branch2\l}\lafterward\l"];
    N1[label="branch1"];
    N2[label="branch2"];
    N3[label="afterward"];
    N0 -> N1[label="then"];
    N0 -> N2[label="else"];
    N1 -> N3[label=";"];
    N2 -> N3[label=";"];
}
"#) ; } # [test] fn simple_id_construction () { let id1 = Id :: new ("hello") ; match id1 { Ok (_) => { } Err (..) => panic ! ("'hello' is not a valid value for id anymore") , } } # [test] fn test_some_arrow () { let labels : Trivial = SomeNodesLabelled (vec ! [Some ("A") , None]) ; let styles = Some (vec ! [Style :: None , Style :: Dotted]) ; let start = Arrow :: default () ; let end = Arrow :: from_arrow (ArrowShape :: crow ()) ; let result = test_input (LabelledGraph :: new ("test_some_labelled" , labels , vec ! [edge_with_arrows (0 , 1 , "A-1" , Style :: None , start , end , None)] , styles ,)) ; assert_eq ! (result . unwrap () , r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="crow"];
}
"#) ; } # [test] fn test_some_arrows () { let labels : Trivial = SomeNodesLabelled (vec ! [Some ("A") , None]) ; let styles = Some (vec ! [Style :: None , Style :: Dotted]) ; let start = Arrow :: from_arrow (ArrowShape :: tee ()) ; let end = Arrow :: from_arrow (ArrowShape :: Crow (Side :: Left)) ; let result = test_input (LabelledGraph :: new ("test_some_labelled" , labels , vec ! [edge_with_arrows (0 , 1 , "A-1" , Style :: None , start , end , None)] , styles ,)) ; assert_eq ! (result . unwrap () , r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="lcrow" dir="both" arrowtail="tee"];
}
"#) ; } # [test] fn badly_formatted_id () { let id2 = Id :: new ("Weird { struct : ure } !!!") ; match id2 { Ok (_) => panic ! ("graphviz id suddenly allows spaces, brackets and stuff") , Err (..) => { } } } type SimpleEdge = (Node , Node) ; struct DefaultStyleGraph { # [doc = " The name for this graph. Used for labelling generated graph"] name : & 'static str , nodes : usize , edges : Vec < SimpleEdge > , kind : Kind , rankdir : Option < RankDir > , } impl DefaultStyleGraph { fn new (name : & 'static str , nodes : usize , edges : Vec < SimpleEdge > , kind : Kind ,) -> DefaultStyleGraph { assert ! (! name . is_empty ()) ; DefaultStyleGraph { name , nodes , edges , kind , rankdir : None , } } fn with_rankdir (self , rankdir : Option < RankDir >) -> Self { Self { rankdir , .. self } } } impl < 'a > Labeller < 'a , Node , & 'a SimpleEdge > for DefaultStyleGraph { fn graph_id (& 'a self) -> Id < 'a > { Id :: new (self . name) . unwrap () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { id_name (n) } fn kind (& self) -> Kind { self . kind } fn rank_dir (& self) -> Option < RankDir > { self . rankdir } } impl < 'a > GraphWalk < 'a , Node , & 'a SimpleEdge > for DefaultStyleGraph { fn nodes (& 'a self) -> Nodes < 'a , Node > { (0 .. self . nodes) . collect () } fn edges (& 'a self) -> Edges < 'a , & 'a SimpleEdge > { self . edges . iter () . collect () } fn source (& 'a self , edge : & & 'a SimpleEdge) -> Node { edge . 0 } fn target (& 'a self , edge : & & 'a SimpleEdge) -> Node { edge . 1 } } fn test_input_default (g : DefaultStyleGraph) -> io :: Result < String > { let mut writer = Vec :: new () ; render (& g , & mut writer) ? ; let mut s = String :: new () ; Read :: read_to_string (& mut & * writer , & mut s) ? ; Ok (s) } # [test] fn default_style_graph () { let r = test_input_default (DefaultStyleGraph :: new ("g" , 4 , vec ! [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)] , Kind :: Graph ,)) ; assert_eq ! (r . unwrap () , r#"graph g {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -- N1[label=""];
    N0 -- N2[label=""];
    N1 -- N3[label=""];
    N2 -- N3[label=""];
}
"#) ; } # [test] fn default_style_digraph () { let r = test_input_default (DefaultStyleGraph :: new ("di" , 4 , vec ! [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)] , Kind :: Digraph ,)) ; assert_eq ! (r . unwrap () , r#"digraph di {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
    N1 -> N3[label=""];
    N2 -> N3[label=""];
}
"#) ; } # [test] fn digraph_with_rankdir () { let r = test_input_default (DefaultStyleGraph :: new ("di" , 4 , vec ! [(0 , 1) , (0 , 2)] , Kind :: Digraph) . with_rankdir (Some (RankDir :: LeftRight)) ,) ; assert_eq ! (r . unwrap () , r#"digraph di {
    rankdir="LR";
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
}
"#) ; } }
};
}
