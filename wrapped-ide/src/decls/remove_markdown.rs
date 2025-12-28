macro_rules! remove_markdown {
    () => {
        # [doc = " Removes all markdown, keeping the text and code blocks"] # [doc = ""] # [doc = " Currently limited in styling, i.e. no ascii tables or lists"] pub (crate) fn remove_markdown (markdown : & str) -> String { let mut out = String :: new () ; out . reserve_exact (markdown . len ()) ; let parser = Parser :: new (markdown) ; for event in parser { match event { Event :: Text (text) | Event :: Code (text) => out . push_str (& text) , Event :: SoftBreak => out . push (' ') , Event :: HardBreak | Event :: Rule | Event :: End (Tag :: CodeBlock (_)) => out . push ('\n') , Event :: End (Tag :: Paragraph) => out . push_str ("\n\n") , Event :: Start (_) | Event :: End (_) | Event :: Html (_) | Event :: FootnoteReference (_) | Event :: TaskListMarker (_) => () , } } if let Some (mut p) = out . rfind (| c | c != '\n') { while ! out . is_char_boundary (p + 1) { p += 1 ; } out . drain (p + 1 ..) ; } out }
    };
}

remove_markdown!();