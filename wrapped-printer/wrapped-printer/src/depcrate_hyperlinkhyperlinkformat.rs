// Generated macro for HyperlinkFormat (struct)
macro_rules! Depcrate_hyperlinkHyperlinkFormat {
() => {
// Module: crate::hyperlink
// Provides: {"HyperlinkFormat"}
// Dependencies: {}
# [doc = " A hyperlink format with variables."] # [doc = ""] # [doc = " This can be created by parsing a string using `HyperlinkFormat::from_str`."] # [doc = ""] # [doc = " The default format is empty. An empty format is valid and effectively"] # [doc = " disables hyperlinks."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use grep_printer::HyperlinkFormat;"] # [doc = ""] # [doc = " let fmt = \"vscode\".parse::<HyperlinkFormat>()?;"] # [doc = " assert_eq!(fmt.to_string(), \"vscode://file{path}:{line}:{column}\");"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct HyperlinkFormat { parts : Vec < Part > , is_line_dependent : bool , }
};
}
