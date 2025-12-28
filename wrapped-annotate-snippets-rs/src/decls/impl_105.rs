macro_rules! deps {
    () => {
        Renderer!();
        DecorStyle!();
        Stylesheet!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl Renderer { # [doc = " No terminal styling"] pub const fn plain () -> Self { Self { anonymized_line_numbers : false , term_width : DEFAULT_TERM_WIDTH , decor_style : DecorStyle :: Ascii , stylesheet : Stylesheet :: plain () , short_message : false , } } # [doc = " Default terminal styling"] # [doc = ""] # [doc = " If ANSI escape codes are not supported, either"] # [doc = " - Call [`Renderer::plain`] instead"] # [doc = " - Strip them after the fact, like with [`anstream`](https://docs.rs/anstream/latest/anstream/)"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " When testing styled terminal output, see the [`testing-colors` feature](crate#features)"] pub const fn styled () -> Self { Self { stylesheet : Stylesheet { error : DEFAULT_ERROR_STYLE , warning : DEFAULT_WARNING_STYLE , info : DEFAULT_INFO_STYLE , note : DEFAULT_NOTE_STYLE , help : DEFAULT_HELP_STYLE , line_num : DEFAULT_LINE_NUM_STYLE , emphasis : DEFAULT_EMPHASIS_STYLE , none : DEFAULT_NONE_STYLE , context : DEFAULT_CONTEXT_STYLE , addition : DEFAULT_ADDITION_STYLE , removal : DEFAULT_REMOVAL_STYLE , } , .. Self :: plain () } } # [doc = " Abbreviate the message"] pub const fn short_message (mut self , short_message : bool) -> Self { self . short_message = short_message ; self } # [doc = " Set the width to render within"] # [doc = ""] # [doc = " Affects the rendering of [`Snippet`][crate::Snippet]s"] pub const fn term_width (mut self , term_width : usize) -> Self { self . term_width = term_width ; self } # [doc = " Set the character set used for rendering decor"] pub const fn decor_style (mut self , decor_style : DecorStyle) -> Self { self . decor_style = decor_style ; self } # [doc = " Anonymize line numbers"] # [doc = ""] # [doc = " When enabled, line numbers are replaced with `LL` which is useful for tests."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```text"] # [doc = "   --> $DIR/whitespace-trimming.rs:4:193"] # [doc = "    |"] # [doc = " LL | ...                   let _: () = 42;"] # [doc = "    |                                   ^^ expected (), found integer"] # [doc = "    |"] # [doc = " ```"] pub const fn anonymized_line_numbers (mut self , anonymized_line_numbers : bool) -> Self { self . anonymized_line_numbers = anonymized_line_numbers ; self } }
    };
}

impl_105!();