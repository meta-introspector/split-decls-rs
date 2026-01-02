mkuse!{use std :: env ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: graph :: linked_graph :: { Direction , INCOMING , NodeIndex , OUTGOING } ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_middle :: dep_graph :: { DepGraphQuery , DepKind , DepNode , DepNodeExt , DepNodeFilter , EdgeFilter , dep_kinds , } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use { rustc_graphviz as dot , rustc_hir as hir } ;}
mkuse!{use crate :: errors ;}

macro_rules! assert_dep_graph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_dep_graph in module {}", module_path!());
    };
}

mkfn!{
    assert_dep_graph_introspect!();
    # [allow (missing_docs)] pub (crate) fn assert_dep_graph (tcx : TyCtxt < '_ >) { tcx . dep_graph . with_ignore (| | { if tcx . sess . opts . unstable_opts . dump_dep_graph { tcx . dep_graph . with_query (dump_graph) ; } if ! tcx . sess . opts . unstable_opts . query_dep_graph { return ; } if ! tcx . features () . rustc_attrs () { return ; } let (if_this_changed , then_this_would_need) = { let mut visitor = IfThisChanged { tcx , if_this_changed : vec ! [] , then_this_would_need : vec ! [] } ; visitor . process_attrs (CRATE_DEF_ID) ; tcx . hir_visit_all_item_likes_in_crate (& mut visitor) ; (visitor . if_this_changed , visitor . then_this_would_need) } ; if ! if_this_changed . is_empty () || ! then_this_would_need . is_empty () { assert ! (tcx . sess . opts . unstable_opts . query_dep_graph , "cannot use the `#[{}]` or `#[{}]` annotations \
                    without supplying `-Z query-dep-graph`" , sym :: rustc_if_this_changed , sym :: rustc_then_this_would_need) ; } check_paths (tcx , & if_this_changed , & then_this_would_need) ; }) }
}
mkitem!{type Sources = Vec < (Span , DefId , DepNode) > ;}
mkitem!{type Targets = Vec < (Span , Symbol , hir :: HirId , DepNode) > ;}
mkitem!{mkstruct!{struct IfThisChanged < 'tcx > { tcx : TyCtxt < 'tcx > , if_this_changed : Sources , then_this_would_need : Targets , }}}
mkitem!{mkimpl!{impl < 'tcx > IfThisChanged < 'tcx > { fn argument (& self , attr : & hir :: Attribute) -> Option < Symbol > { let mut value = None ; for list_item in attr . meta_item_list () . unwrap_or_default () { match list_item . ident () { Some (ident) if list_item . is_word () && value . is_none () => value = Some (ident . name) , _ => { span_bug ! (list_item . span () , "unexpected meta-item {:?}" , list_item) } } } value } fn process_attrs (& mut self , def_id : LocalDefId) { let def_path_hash = self . tcx . def_path_hash (def_id . to_def_id ()) ; let hir_id = self . tcx . local_def_id_to_hir_id (def_id) ; let attrs = self . tcx . hir_attrs (hir_id) ; for attr in attrs { if attr . has_name (sym :: rustc_if_this_changed) { let dep_node_interned = self . argument (attr) ; let dep_node = match dep_node_interned { None => DepNode :: from_def_path_hash (self . tcx , def_path_hash , dep_kinds :: opt_hir_owner_nodes ,) , Some (n) => { match DepNode :: from_label_string (self . tcx , n . as_str () , def_path_hash) { Ok (n) => n , Err (()) => self . tcx . dcx () . emit_fatal (errors :: UnrecognizedDepNode { span : attr . span () , name : n , }) , } } } ; self . if_this_changed . push ((attr . span () , def_id . to_def_id () , dep_node)) ; } else if attr . has_name (sym :: rustc_then_this_would_need) { let dep_node_interned = self . argument (attr) ; let dep_node = match dep_node_interned { Some (n) => { match DepNode :: from_label_string (self . tcx , n . as_str () , def_path_hash) { Ok (n) => n , Err (()) => self . tcx . dcx () . emit_fatal (errors :: UnrecognizedDepNode { span : attr . span () , name : n , }) , } } None => { self . tcx . dcx () . emit_fatal (errors :: MissingDepNode { span : attr . span () }) ; } } ; self . then_this_would_need . push ((attr . span () , dep_node_interned . unwrap () , hir_id , dep_node ,)) ; } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for IfThisChanged < 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_item (& mut self , item : & 'tcx hir :: Item < 'tcx >) { self . process_attrs (item . owner_id . def_id) ; intravisit :: walk_item (self , item) ; } fn visit_trait_item (& mut self , trait_item : & 'tcx hir :: TraitItem < 'tcx >) { self . process_attrs (trait_item . owner_id . def_id) ; intravisit :: walk_trait_item (self , trait_item) ; } fn visit_impl_item (& mut self , impl_item : & 'tcx hir :: ImplItem < 'tcx >) { self . process_attrs (impl_item . owner_id . def_id) ; intravisit :: walk_impl_item (self , impl_item) ; } fn visit_field_def (& mut self , s : & 'tcx hir :: FieldDef < 'tcx >) { self . process_attrs (s . def_id) ; intravisit :: walk_field_def (self , s) ; } }}}

macro_rules! check_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_paths in module {}", module_path!());
    };
}

mkfn!{
    check_paths_introspect!();
    fn check_paths < 'tcx > (tcx : TyCtxt < 'tcx > , if_this_changed : & Sources , then_this_would_need : & Targets) { if if_this_changed . is_empty () { for & (target_span , _ , _ , _) in then_this_would_need { tcx . dcx () . emit_err (errors :: MissingIfThisChanged { span : target_span }) ; } return ; } tcx . dep_graph . with_query (| query | { for & (_ , source_def_id , ref source_dep_node) in if_this_changed { let dependents = query . transitive_predecessors (source_dep_node) ; for & (target_span , ref target_pass , _ , ref target_dep_node) in then_this_would_need { if ! dependents . contains (& target_dep_node) { tcx . dcx () . emit_err (errors :: NoPath { span : target_span , source : tcx . def_path_str (source_def_id) , target : * target_pass , }) ; } else { tcx . dcx () . emit_err (errors :: Ok { span : target_span }) ; } } } }) ; }
}

macro_rules! dump_graph_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dump_graph in module {}", module_path!());
    };
}

mkfn!{
    dump_graph_introspect!();
    fn dump_graph (query : & DepGraphQuery) { let path : String = env :: var ("RUST_DEP_GRAPH") . unwrap_or_else (| _ | "dep_graph" . to_string ()) ; let nodes = match env :: var ("RUST_DEP_GRAPH_FILTER") { Ok (string) => { let edge_filter = EdgeFilter :: new (& string) . unwrap_or_else (| e | bug ! ("invalid filter: {}" , e)) ; let sources = node_set (query , & edge_filter . source) ; let targets = node_set (query , & edge_filter . target) ; filter_nodes (query , & sources , & targets) } Err (_) => query . nodes () . into_iter () . map (| n | n . kind) . collect () , } ; let edges = filter_edges (query , & nodes) ; { let txt_path = format ! ("{path}.txt") ; let mut file = File :: create_buffered (& txt_path) . unwrap () ; for (source , target) in & edges { write ! (file , "{source:?} -> {target:?}\n") . unwrap () ; } } { let dot_path = format ! ("{path}.dot") ; let mut v = Vec :: new () ; dot :: render (& GraphvizDepGraph (nodes , edges) , & mut v) . unwrap () ; fs :: write (dot_path , v) . unwrap () ; } }
}
mkitem!{mkstruct!{# [allow (missing_docs)] struct GraphvizDepGraph (FxIndexSet < DepKind > , Vec < (DepKind , DepKind) >) ;}}
mkitem!{mkimpl!{impl < 'a > dot :: GraphWalk < 'a > for GraphvizDepGraph { type Node = DepKind ; type Edge = (DepKind , DepKind) ; fn nodes (& self) -> dot :: Nodes < '_ , DepKind > { let nodes : Vec < _ > = self . 0 . iter () . cloned () . collect () ; nodes . into () } fn edges (& self) -> dot :: Edges < '_ , (DepKind , DepKind) > { self . 1 [..] . into () } fn source (& self , edge : & (DepKind , DepKind)) -> DepKind { edge . 0 } fn target (& self , edge : & (DepKind , DepKind)) -> DepKind { edge . 1 } }}}
mkitem!{mkimpl!{impl < 'a > dot :: Labeller < 'a > for GraphvizDepGraph { type Node = DepKind ; type Edge = (DepKind , DepKind) ; fn graph_id (& self) -> dot :: Id < '_ > { dot :: Id :: new ("DependencyGraph") . unwrap () } fn node_id (& self , n : & DepKind) -> dot :: Id < '_ > { let s : String = format ! ("{n:?}") . chars () . map (| c | if c == '_' || c . is_alphanumeric () { c } else { '_' }) . collect () ; debug ! ("n={:?} s={:?}" , n , s) ; dot :: Id :: new (s) . unwrap () } fn node_label (& self , n : & DepKind) -> dot :: LabelText < '_ > { dot :: LabelText :: label (format ! ("{n:?}")) } }}}

macro_rules! node_set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function node_set in module {}", module_path!());
    };
}

mkfn!{
    node_set_introspect!();
    fn node_set < 'q > (query : & 'q DepGraphQuery , filter : & DepNodeFilter ,) -> Option < FxIndexSet < & 'q DepNode > > { debug ! ("node_set(filter={:?})" , filter) ; if filter . accepts_all () { return None ; } Some (query . nodes () . into_iter () . filter (| n | filter . test (n)) . collect ()) }
}

macro_rules! filter_nodes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filter_nodes in module {}", module_path!());
    };
}

mkfn!{
    filter_nodes_introspect!();
    fn filter_nodes < 'q > (query : & 'q DepGraphQuery , sources : & Option < FxIndexSet < & 'q DepNode > > , targets : & Option < FxIndexSet < & 'q DepNode > > ,) -> FxIndexSet < DepKind > { if let Some (sources) = sources { if let Some (targets) = targets { walk_between (query , sources , targets) } else { walk_nodes (query , sources , OUTGOING) } } else if let Some (targets) = targets { walk_nodes (query , targets , INCOMING) } else { query . nodes () . into_iter () . map (| n | n . kind) . collect () } }
}

macro_rules! walk_nodes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_nodes in module {}", module_path!());
    };
}

mkfn!{
    walk_nodes_introspect!();
    fn walk_nodes < 'q > (query : & 'q DepGraphQuery , starts : & FxIndexSet < & 'q DepNode > , direction : Direction ,) -> FxIndexSet < DepKind > { let mut set = FxIndexSet :: default () ; for & start in starts { debug ! ("walk_nodes: start={:?} outgoing?={:?}" , start , direction == OUTGOING) ; if set . insert (start . kind) { let mut stack = vec ! [query . indices [start]] ; while let Some (index) = stack . pop () { for (_ , edge) in query . graph . adjacent_edges (index , direction) { let neighbor_index = edge . source_or_target (direction) ; let neighbor = query . graph . node_data (neighbor_index) ; if set . insert (neighbor . kind) { stack . push (neighbor_index) ; } } } } } set }
}

macro_rules! walk_between_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_between in module {}", module_path!());
    };
}

mkfn!{
    walk_between_introspect!();
    fn walk_between < 'q > (query : & 'q DepGraphQuery , sources : & FxIndexSet < & 'q DepNode > , targets : & FxIndexSet < & 'q DepNode > ,) -> FxIndexSet < DepKind > { # [derive (Copy , Clone , PartialEq)] enum State { Undecided , Deciding , Included , Excluded , } let mut node_states = vec ! [State :: Undecided ; query . graph . len_nodes ()] ; for & target in targets { node_states [query . indices [target] . 0] = State :: Included ; } for source in sources . iter () . map (| & n | query . indices [n]) { recurse (query , & mut node_states , source) ; } return query . nodes () . into_iter () . filter (| & n | { let index = query . indices [n] ; node_states [index . 0] == State :: Included }) . map (| n | n . kind) . collect () ; fn recurse (query : & DepGraphQuery , node_states : & mut [State] , node : NodeIndex) -> bool { match node_states [node . 0] { State :: Included => return true , State :: Excluded => return false , State :: Deciding => return false , State :: Undecided => { } } node_states [node . 0] = State :: Deciding ; for neighbor_index in query . graph . successor_nodes (node) { if recurse (query , node_states , neighbor_index) { node_states [node . 0] = State :: Included ; } } if node_states [node . 0] == State :: Deciding { node_states [node . 0] = State :: Excluded ; false } else { assert ! (node_states [node . 0] == State :: Included) ; true } } }
}

macro_rules! filter_edges_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filter_edges in module {}", module_path!());
    };
}

mkfn!{
    filter_edges_introspect!();
    fn filter_edges (query : & DepGraphQuery , nodes : & FxIndexSet < DepKind >) -> Vec < (DepKind , DepKind) > { let uniq : FxIndexSet < _ > = query . edges () . into_iter () . map (| (s , t) | (s . kind , t . kind)) . filter (| (source , target) | nodes . contains (source) && nodes . contains (target)) . collect () ; uniq . into_iter () . collect () }
}