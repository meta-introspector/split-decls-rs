// Generated macro for rustc_lean4_bridge_impl (function)
macro_rules! Depcrate_lean4_jsonrustc_lean4_bridge_impl {
() => {
// Module: crate::lean4_json
// Provides: {"rustc_lean4_bridge_impl"}
// Dependencies: {}
# [decl (fn , name = "rustc_lean4_bridge_impl" , vis = "pub" , hash = "f11b601c")] pub fn rustc_lean4_bridge_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let rustc_structure = input_str . value () ; quote ! { { println ! ("cargo:warning=🌉 Creating Rustc→Lean4 JSON bridge") ; let bridge_json = format ! (r#"{{
  "bridge": "rustc_to_lean4",
  "source": {{
    "language": "rust",
    "structure": "{}",
    "compiler": "rustc"
  }},
  "target": {{
    "language": "lean4",
    "type_theory": "dependent_types",
    "proof_assistant": true
  }},
  "mapping": {{
    "struct_to_structure": true,
    "impl_to_instance": true,
    "fn_to_def": true,
    "type_to_type": true
  }},
  "monster_embedding": {{
    "group_dimension": 196883,
    "morphism": "conformal_field_theory",
    "lfunction_unity": true
  }},
  "json_serialization": {{
    "expr_objects": true,
    "ast_preservation": true,
    "type_information": true
  }}
}}"# , # rustc_structure) ; bridge_json } } . into () }
};
}
