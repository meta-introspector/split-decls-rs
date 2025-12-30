// Generated macro for admonitions (function)
macro_rules! Depcrate_admonitionsadmonitions {
() => {
// Module: crate::admonitions
// Provides: {"admonitions"}
// Dependencies: {}
# [doc = " Converts blockquotes with special headers into admonitions."] # [doc = ""] # [doc = " The blockquote should look something like:"] # [doc = ""] # [doc = " ```markdown"] # [doc = " > [!WARNING]"] # [doc = " > ..."] # [doc = " ```"] # [doc = ""] # [doc = " This will add a `<div class=\"alert alert-warning\">` around the"] # [doc = " blockquote so that it can be styled differently, and injects an icon."] # [doc = " The actual styling needs to be added in the `reference.css` CSS file."] pub fn admonitions (chapter : & Chapter , diag : & mut Diagnostics) -> String { ADMONITION_RE . replace_all (& chapter . content , | caps : & Captures < '_ > | { let lower = caps ["admon"] . to_lowercase () ; let term = to_initial_case (& caps ["admon"]) ; let blockquote = & caps ["blockquote"] ; let initial_spaces = blockquote . chars () . position (| ch | ch != ' ') . unwrap_or (0) ; let space = & blockquote [.. initial_spaces] ; let format_div = | class , content | { format ! ("{space}<div class=\"alert alert-{class}\">\n\
                    \n\
                    {space}> <p class=\"alert-title\">\
                        {content}</p>\n\
                    {space} >\n\
                    {blockquote}\n\
                    \n\
                    {space}</div>\n" ,) } ; if lower . starts_with ("edition-") { let edition = & lower [8 ..] ; return format_div ("edition" , format ! ("<span class=\"alert-title-edition\">{edition}</span> Edition differences") ,) ; } let svg = match lower . as_str () { "note" => ICON_NOTE , "warning" => ICON_WARNING , "example" => ICON_EXAMPLE , _ => { warn_or_err ! (diag , "admonition `{lower}` in {:?} is incorrect or not yet supported" , chapter . path . as_ref () . unwrap ()) ; "" } } ; format_div (& lower , format ! ("<svg viewBox=\"0 0 16 16\" width=\"18\" height=\"18\">\
                        {svg}\
                    </svg>{term}") ,) }) . to_string () }
};
}
