macro_rules! deps {
    () => {
        Result!();
        Context!();
        Node!();
        SpanError!();
        Error!();
        Color!();
        ColorTag!();
    };
}

macro_rules! parse_format_string {
    () => {
        deps!();
        # [doc = " Parses a format string which may contain usual format placeholders (`{...}`) as well as color"] # [doc = " codes like `\"<red>\"`, `\"<blue,bold>\"`."] pub fn parse_format_string < 'a > (input : & 'a str , lit_str : & LitStr ,) -> Result < Vec < Node < 'a > > , SpanError > { # [doc = " Representation of the parsing context. Each variant's argument is the start offset of the"] # [doc = " given parse context."] enum Context { # [doc = " The char actually parsed is a textual character:"] Text (usize) , # [doc = " The char actually parsed is part of a `format!`-like placeholder:"] Placeholder (usize) , # [doc = " The char actually parsed is part of a color tag, like `<red>`:"] Color (usize) , } macro_rules ! span { ($ inside : expr) => { inner_span (input , lit_str , $ inside) } ; } macro_rules ! err { ([$ inside : expr] $ ($ e : tt) *) => { SpanError :: new ($ ($ e) *, Some (span ! ($ inside))) } ; ($ ($ e : tt) *) => { SpanError :: new ($ ($ e) *, Some (lit_str . span ())) } ; } let mut context = Context :: Text (0) ; let mut nodes = vec ! [] ; let mut close_angle_bracket_idx : Option < usize > = None ; let mut nb_open_tags : isize = 0 ; for (i , c) in input . char_indices () { match context { Context :: Text (text_start) => { let mut push_text = false ; if c == '{' { context = Context :: Placeholder (i) ; push_text = true ; } else if c == '<' { context = Context :: Color (i) ; push_text = true ; } else if c == '>' { if let Some (idx) = close_angle_bracket_idx { if i == idx + 1 { context = Context :: Text (i + 1) ; push_text = true ; } close_angle_bracket_idx = None ; } else { close_angle_bracket_idx = Some (i) ; } } ; if push_text && text_start != i { nodes . push (Node :: Text (& input [text_start .. i])) ; } } Context :: Placeholder (ph_start) => { if c == '{' && i == ph_start + 1 { context = Context :: Text (ph_start) ; } else if c == '}' { nodes . push (Node :: Placeholder (& input [ph_start .. i + 1])) ; context = Context :: Text (i + 1) ; } } Context :: Color (tag_start) => { if c == '<' && i == tag_start + 1 { context = Context :: Text (tag_start + 1) ; } else if c == '>' { let tag_input = & input [tag_start .. i + 1] ; let mut tag = parse :: color_tag (tag_input) . map_err (| e | { use nom :: Err ; let (input , error) = match e { Err :: Error (parse :: Error { detail : Some (d) , .. }) | Err :: Failure (parse :: Error { detail : Some (d) , .. }) => { (d . input , Error :: ParseTag (d . message)) } _ => (tag_input , Error :: UnableToParseTag (tag_input . to_string ())) , } ; err ! ([input] error) }) ? . 1 ; tag . set_span (span ! (tag_input)) ; nb_open_tags += if tag . is_close { - 1 } else { 1 } ; if let Some (Node :: ColorTagGroup (last_tag_group)) = nodes . last_mut () { last_tag_group . push (tag) ; } else { nodes . push (Node :: ColorTagGroup (vec ! [tag])) ; } context = Context :: Text (i + 1) ; } } } } match context { Context :: Text (text_start) => { if text_start != input . len () { nodes . push (Node :: Text (& input [text_start ..])) ; } if nb_open_tags > 0 { let tags = (0 .. nb_open_tags) . map (| _ | ColorTag :: new_close ()) . collect :: < Vec < _ > > () ; nodes . push (Node :: ColorTagGroup (tags)) ; } Ok (nodes) } Context :: Placeholder (start) => Err (err ! ([& input [start ..]] Error :: UnclosedPlaceholder)) , Context :: Color (start) => Err (err ! ([& input [start ..]] Error :: UnclosedTag)) , } }
    };
}

parse_format_string!();