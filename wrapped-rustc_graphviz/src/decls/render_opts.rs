macro_rules! deps {
    () => {
        Style!();
        Labeller!();
        Edge!();
        Node!();
        GraphWalk!();
        RenderOption!();
    };
}

macro_rules! render_opts {
    () => {
        deps!();
        # [doc = " Renders directed graph `g` into the writer `w` in DOT syntax."] # [doc = " (Main entry point for the library.)"] pub fn render_opts < 'a , N , E , G , W > (g : & 'a G , w : & mut W , options : & [RenderOption]) -> io :: Result < () > where N : Clone + 'a , E : Clone + 'a , G : Labeller < 'a , Node = N , Edge = E > + GraphWalk < 'a , Node = N , Edge = E > , W : Write , { writeln ! (w , "digraph {} {{" , g . graph_id () . as_slice ()) ? ; let mut graph_attrs = Vec :: new () ; let mut content_attrs = Vec :: new () ; let font ; if let Some (fontname) = options . iter () . find_map (| option | { if let RenderOption :: Fontname (fontname) = option { Some (fontname) } else { None } }) { font = format ! (r#"fontname="{fontname}""#) ; graph_attrs . push (& font [..]) ; content_attrs . push (& font [..]) ; } if options . contains (& RenderOption :: DarkTheme) { graph_attrs . push (r#"bgcolor="black""#) ; graph_attrs . push (r#"fontcolor="white""#) ; content_attrs . push (r#"color="white""#) ; content_attrs . push (r#"fontcolor="white""#) ; } if ! (graph_attrs . is_empty () && content_attrs . is_empty ()) { writeln ! (w , r#"    graph[{}];"# , graph_attrs . join (" ")) ? ; let content_attrs_str = content_attrs . join (" ") ; writeln ! (w , r#"    node[{content_attrs_str}];"#) ? ; writeln ! (w , r#"    edge[{content_attrs_str}];"#) ? ; } let mut text = Vec :: new () ; for n in g . nodes () . iter () { write ! (w , "    ") ? ; let id = g . node_id (n) ; let escaped = & g . node_label (n) . to_dot_string () ; write ! (text , "{}" , id . as_slice ()) . unwrap () ; if ! options . contains (& RenderOption :: NoNodeLabels) { write ! (text , "[label={escaped}]") . unwrap () ; } let style = g . node_style (n) ; if ! options . contains (& RenderOption :: NoNodeStyles) && style != Style :: None { write ! (text , "[style=\"{}\"]" , style . as_slice ()) . unwrap () ; } if let Some (s) = g . node_shape (n) { write ! (text , "[shape={}]" , & s . to_dot_string ()) . unwrap () ; } writeln ! (text , ";") . unwrap () ; w . write_all (& text) ? ; text . clear () ; } for e in g . edges () . iter () { let escaped_label = & g . edge_label (e) . to_dot_string () ; write ! (w , "    ") ? ; let source = g . source (e) ; let target = g . target (e) ; let source_id = g . node_id (& source) ; let target_id = g . node_id (& target) ; write ! (text , "{} -> {}" , source_id . as_slice () , target_id . as_slice ()) . unwrap () ; if ! options . contains (& RenderOption :: NoEdgeLabels) { write ! (text , "[label={escaped_label}]") . unwrap () ; } let style = g . edge_style (e) ; if ! options . contains (& RenderOption :: NoEdgeStyles) && style != Style :: None { write ! (text , "[style=\"{}\"]" , style . as_slice ()) . unwrap () ; } writeln ! (text , ";") . unwrap () ; w . write_all (& text) ? ; text . clear () ; } writeln ! (w , "}}") }
    };
}

render_opts!();