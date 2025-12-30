// Generated macro for str_literal_to_char_literal (function)
macro_rules! Depcrate_sourcestr_literal_to_char_literal {
() => {
// Module: crate::source
// Provides: {"str_literal_to_char_literal"}
// Dependencies: {}
# [doc = " Converts `expr` to a `char` literal if it's a `str` literal containing a single"] # [doc = " character (or a single byte with `ascii_only`)"] pub fn str_literal_to_char_literal (sess : & impl HasSession , expr : & Expr < '_ > , applicability : & mut Applicability , ascii_only : bool ,) -> Option < String > { if let ExprKind :: Lit (lit) = & expr . kind && let LitKind :: Str (r , style) = lit . node && let string = r . as_str () && let len = if ascii_only { string . len () } else { string . chars () . count () } && len == 1 { let snip = snippet_with_applicability (sess , expr . span , string , applicability) ; let ch = if let StrStyle :: Raw (nhash) = style { let nhash = nhash as usize ; & snip [(nhash + 2) .. (snip . len () - 1 - nhash)] } else { & snip [1 .. (snip . len () - 1)] } ; let hint = format ! ("'{}'" , match ch { "'" => "\\'" , r"\" => "\\\\" , "\\\"" => "\"" , _ => ch , }) ; Some (hint) } else { None } }
};
}
