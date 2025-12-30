// Generated macro for impl_63 (impl)
macro_rules! Depcrate_metadataimpl_63 {
() => {
// Module: crate::metadata
// Provides: {"impl_63"}
// Dependencies: {}
impl ClippyConfiguration { pub fn to_markdown_paragraph (& self) -> String { format ! ("## `{}`\n{}\n\n**Default Value:** `{}`\n\n---\n**Affected lints:**\n{}\n\n" , self . name , self . doc . lines () . map (| x | x . strip_prefix (' ') . unwrap_or (x)) . join ("\n") , self . default , self . lints . iter () . format_with ("\n" , | name , f | f (& format_args ! ("* [`{name}`](https://rust-lang.github.io/rust-clippy/master/index.html#{name})"))) ,) } pub fn to_markdown_link (& self) -> String { const BOOK_CONFIGS_PATH : & str = "https://doc.rust-lang.org/clippy/lint_configuration.html" ; format ! ("[`{}`]: {BOOK_CONFIGS_PATH}#{}" , self . name , self . name) } }
};
}
