// Generated macro for lean4_expr_json_impl (function)
macro_rules! Depcrate_lean4_jsonlean4_expr_json_impl {
() => {
// Module: crate::lean4_json
// Provides: {"lean4_expr_json_impl"}
// Dependencies: {}
# [decl (fn , name = "lean4_expr_json_impl" , vis = "pub" , hash = "45fba3f5")] pub fn lean4_expr_json_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let lean4_expr = input_str . value () ; quote ! { { println ! ("cargo:warning=📄 Converting Lean4 Expr to JSON") ; let json_expr = format ! (r#"{{
  "type": "Lean4Expr",
  "original": "{}",
  "ast": {{
    "kind": "{}",
    "name": "{}",
    "type": "{}",
    "args": [{}],
    "body": "{}"
  }},
  "serialization": {{
    "format": "json",
    "version": "4.0",
    "timestamp": {}
  }}
}}"# , # lean4_expr , if # lean4_expr . contains ("theorem") { "Theorem" } else if # lean4_expr . contains ("def") { "Definition" } else if # lean4_expr . contains ("structure") { "Structure" } else { "Expression" } , # lean4_expr . split_whitespace () . nth (1) . unwrap_or ("unknown") , if # lean4_expr . contains (":") { # lean4_expr . split (':') . nth (1) . unwrap_or ("Type") . trim () } else { "Type" } , if # lean4_expr . contains ("(") { format ! ("\"{}\"" , # lean4_expr . split ('(') . nth (1) . unwrap_or ("") . split (')') . next () . unwrap_or ("")) } else { "" . to_string () } , if # lean4_expr . contains (":=") { # lean4_expr . split (":=") . nth (1) . unwrap_or ("") . trim () } else { "" } , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs ()) ; json_expr } } . into () }
};
}
