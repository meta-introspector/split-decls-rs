mkuse!{use std :: io :: { self , Write } ;}
mkuse!{use gsgdt :: GraphvizSettings ;}
mkuse!{use rustc_graphviz as dot ;}
mkuse!{use super :: generic_graph :: mir_fn_to_generic_graph ;}
mkuse!{use super :: pretty :: dump_mir_def_ids ;}
mkuse!{use crate :: mir :: * ;}

macro_rules! write_mir_graphviz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_mir_graphviz in module {}", module_path!());
    };
}

mkfn!{
    write_mir_graphviz_introspect!();
    # [doc = " Write a graphviz DOT graph of a list of MIRs."] pub fn write_mir_graphviz < W > (tcx : TyCtxt < '_ > , single : Option < DefId > , w : & mut W) -> io :: Result < () > where W : Write , { let def_ids = dump_mir_def_ids (tcx , single) ; let mirs = def_ids . iter () . flat_map (| def_id | { if tcx . is_const_fn (* def_id) { vec ! [tcx . optimized_mir (* def_id) , tcx . mir_for_ctfe (* def_id)] } else { vec ! [tcx . instance_mir (ty :: InstanceKind :: Item (* def_id))] } }) . collect :: < Vec < _ > > () ; let use_subgraphs = mirs . len () > 1 ; if use_subgraphs { writeln ! (w , "digraph __crate__ {{") ? ; } for mir in mirs { write_mir_fn_graphviz (tcx , mir , use_subgraphs , w) ? ; } if use_subgraphs { writeln ! (w , "}}") ? ; } Ok (()) }
}

macro_rules! write_mir_fn_graphviz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_mir_fn_graphviz in module {}", module_path!());
    };
}

mkfn!{
    write_mir_fn_graphviz_introspect!();
    # [doc = " Write a graphviz DOT graph of the MIR."] pub fn write_mir_fn_graphviz < 'tcx , W > (tcx : TyCtxt < 'tcx > , body : & Body < '_ > , subgraph : bool , w : & mut W ,) -> io :: Result < () > where W : Write , { let font = format ! (r#"fontname="{}""# , tcx . sess . opts . unstable_opts . graphviz_font) ; let mut graph_attrs = vec ! [& font [..]] ; let mut content_attrs = vec ! [& font [..]] ; let dark_mode = tcx . sess . opts . unstable_opts . graphviz_dark_mode ; if dark_mode { graph_attrs . push (r#"bgcolor="black""#) ; graph_attrs . push (r#"fontcolor="white""#) ; content_attrs . push (r#"color="white""#) ; content_attrs . push (r#"fontcolor="white""#) ; } let mut label = String :: from ("") ; write_graph_label (tcx , body , & mut label) . unwrap () ; let g = mir_fn_to_generic_graph (tcx , body) ; let settings = GraphvizSettings { graph_attrs : Some (graph_attrs . join (" ")) , node_attrs : Some (content_attrs . join (" ")) , edge_attrs : Some (content_attrs . join (" ")) , graph_label : Some (label) , } ; g . to_dot (w , & settings , subgraph) }
}

macro_rules! write_graph_label_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_graph_label in module {}", module_path!());
    };
}

mkfn!{
    write_graph_label_introspect!();
    # [doc = " Write the graphviz DOT label for the overall graph. This is essentially a block of text that"] # [doc = " will appear below the graph, showing the type of the `fn` this MIR represents and the types of"] # [doc = " all the variables and temporaries."] fn write_graph_label < 'tcx , W : std :: fmt :: Write > (tcx : TyCtxt < 'tcx > , body : & Body < '_ > , w : & mut W ,) -> std :: fmt :: Result { let def_id = body . source . def_id () ; write ! (w , "fn {}(" , dot :: escape_html (& tcx . def_path_str (def_id))) ? ; for (i , arg) in body . args_iter () . enumerate () { if i > 0 { write ! (w , ", ") ? ; } write ! (w , "{:?}: {}" , Place :: from (arg) , escape (& body . local_decls [arg] . ty)) ? ; } write ! (w , ") -&gt; {}" , escape (& body . return_ty ())) ? ; write ! (w , r#"<br align="left"/>"#) ? ; for local in body . vars_and_temps_iter () { let decl = & body . local_decls [local] ; write ! (w , "let ") ? ; if decl . mutability . is_mut () { write ! (w , "mut ") ? ; } write ! (w , r#"{:?}: {};<br align="left"/>"# , Place :: from (local) , escape (& decl . ty)) ? ; } for var_debug_info in & body . var_debug_info { write ! (w , r#"debug {} =&gt; {};<br align="left"/>"# , var_debug_info . name , escape (& var_debug_info . value) ,) ? ; } Ok (()) }
}

macro_rules! escape_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape in module {}", module_path!());
    };
}

mkfn!{
    escape_introspect!();
    fn escape < T : Debug > (t : & T) -> String { dot :: escape_html (& format ! ("{t:?}")) }
}