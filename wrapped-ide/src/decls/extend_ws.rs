macro_rules! extend_ws {
    () => {
        fn extend_ws (root : & SyntaxNode , ws : SyntaxToken , offset : TextSize) -> TextRange { let ws_text = ws . text () ; let suffix = TextRange :: new (offset , ws . text_range () . end ()) - ws . text_range () . start () ; let prefix = TextRange :: new (ws . text_range () . start () , offset) - ws . text_range () . start () ; let ws_suffix = & ws_text [suffix] ; let ws_prefix = & ws_text [prefix] ; if ws_text . contains ('\n') && ! ws_suffix . contains ('\n') && let Some (node) = ws . next_sibling_or_token () { let start = match ws_prefix . rfind ('\n') { Some (idx) => ws . text_range () . start () + TextSize :: from ((idx + 1) as u32) , None => node . text_range () . start () , } ; let end = if root . text () . char_at (node . text_range () . end ()) == Some ('\n') { node . text_range () . end () + TextSize :: of ('\n') } else { node . text_range () . end () } ; return TextRange :: new (start , end) ; } ws . text_range () }
    };
}

extend_ws!()