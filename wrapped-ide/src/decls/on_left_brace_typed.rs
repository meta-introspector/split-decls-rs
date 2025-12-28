macro_rules! on_left_brace_typed {
    () => {
        fn on_left_brace_typed (reparsed : & SourceFile , offset : TextSize) -> Option < TextEdit > { let segment : ast :: PathSegment = find_node_at_offset (reparsed . syntax () , offset) ? ; if segment . syntax () . text_range () . start () != offset { return None ; } let tree : ast :: UseTree = find_node_at_offset (reparsed . syntax () , offset) ? ; Some (TextEdit :: insert (tree . syntax () . text_range () . end () + TextSize :: of ("{") , "}" . to_owned ())) }
    };
}

on_left_brace_typed!()