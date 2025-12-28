macro_rules! on_pipe_typed {
    () => {
        fn on_pipe_typed (file : & SourceFile , offset : TextSize) -> Option < TextEdit > { let pipe_token = file . syntax () . token_at_offset (offset) . right_biased () ? ; if pipe_token . kind () != SyntaxKind :: PIPE { return None ; } if pipe_token . parent () . and_then (ast :: ParamList :: cast) ? . r_paren_token () . is_some () { return None ; } let after_lpipe = offset + TextSize :: of ('|') ; Some (TextEdit :: insert (after_lpipe , "|" . to_owned ())) }
    };
}

on_pipe_typed!();