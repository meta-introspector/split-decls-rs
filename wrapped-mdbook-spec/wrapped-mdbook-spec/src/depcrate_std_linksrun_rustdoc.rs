// Generated macro for run_rustdoc (function)
macro_rules! Depcrate_std_linksrun_rustdoc {
() => {
// Module: crate::std_links
// Provides: {"run_rustdoc"}
// Dependencies: {}
# [doc = " Generates links using rustdoc."] # [doc = ""] # [doc = " This takes the given links and creates a temporary Rust source file"] # [doc = " containing those links within doc-comments, and then runs rustdoc to"] # [doc = " generate intra-doc links on them."] # [doc = ""] # [doc = " The output will be in the given `tmp` directory."] fn run_rustdoc (tmp : & TempDir , chapter_links : & HashMap < & PathBuf , Vec < Link < '_ > > > , diag : & mut Diagnostics ,) -> Result < () > { let src_path = tmp . path () . join ("a.rs") ; let mut src = format ! ("#![{}(rustdoc::broken_intra_doc_links)]\n\
         #![allow(rustdoc::redundant_explicit_links)]\n" , if diag . deny_warnings { "deny" } else { "warn" }) ; for (_ch_path , links) in chapter_links { for link in links { match link . link_type { LinkType :: Inline | LinkType :: Reference | LinkType :: Collapsed | LinkType :: Shortcut => { writeln ! (src , "//! - LINK: [{}]" , link . dest_url) . unwrap () ; } LinkType :: ReferenceUnknown | LinkType :: CollapsedUnknown | LinkType :: ShortcutUnknown => { bug ! ("unexpected link type unknown {link:?}") ; } LinkType :: Autolink | LinkType :: Email => { bug ! ("link type should have been filtered {link:?}") ; } LinkType :: WikiLink { .. } => panic ! ("unsupported wikilink") , } } } writeln ! (src , "extern crate alloc;\n\
         extern crate proc_macro;\n\
         extern crate test;\n") . unwrap () ; fs :: write (& src_path , & src) . unwrap () ; let rustdoc = std :: env :: var ("RUSTDOC") . unwrap_or_else (| _ | "rustdoc" . into ()) ; let output = Command :: new (rustdoc) . arg ("--edition=2024") . arg (& src_path) . current_dir (tmp . path ()) . output () . expect ("rustdoc installed") ; if ! output . status . success () { let stderr = String :: from_utf8_lossy (& output . stderr) ; bail ! ("failed to extract std links ({:?})\n{stderr}" , output . status) ; } Ok (()) }
};
}
