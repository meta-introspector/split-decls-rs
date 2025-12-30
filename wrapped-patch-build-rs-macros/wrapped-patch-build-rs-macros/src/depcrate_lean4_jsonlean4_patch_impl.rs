// Generated macro for lean4_patch_impl (function)
macro_rules! Depcrate_lean4_jsonlean4_patch_impl {
() => {
// Module: crate::lean4_json
// Provides: {"lean4_patch_impl"}
// Dependencies: {}
# [decl (fn , name = "lean4_patch_impl" , vis = "pub" , hash = "1ebce52b")] pub fn lean4_patch_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let patch_description = input_str . value () ; quote ! { { println ! ("cargo:warning=🔧 Generating Lean4 patch for JSON export") ; let lean4_patch = format ! (r#"
-- Lean4 Patch: {}
-- Adds JSON serialization for Expr objects

import Lean.Data.Json
import Lean.Expr

namespace Lean.Expr

-- JSON serialization for Expr
def toJson (e : Expr) : Json :=
  match e with
  | .bvar idx => Json.mkObj [("type", "bvar"), ("index", Json.num idx)]
  | .fvar id => Json.mkObj [("type", "fvar"), ("id", Json.str id.name)]
  | .mvar id => Json.mkObj [("type", "mvar"), ("id", Json.str id.name)]
  | .sort lvl => Json.mkObj [("type", "sort"), ("level", Json.str (toString lvl))]
  | .const name lvls => Json.mkObj [
      ("type", "const"), 
      ("name", Json.str (toString name)),
      ("levels", Json.arr (lvls.map (λ l => Json.str (toString l))))
    ]
  | .app fn arg => Json.mkObj [
      ("type", "app"),
      ("function", fn.toJson),
      ("argument", arg.toJson)
    ]
  | .lam name type body info => Json.mkObj [
      ("type", "lambda"),
      ("name", Json.str (toString name)),
      ("type", type.toJson),
      ("body", body.toJson),
      ("info", Json.str (toString info))
    ]
  | .forallE name type body info => Json.mkObj [
      ("type", "forall"),
      ("name", Json.str (toString name)),
      ("type", type.toJson),
      ("body", body.toJson),
      ("info", Json.str (toString info))
    ]
  | .letE name type value body _ => Json.mkObj [
      ("type", "let"),
      ("name", Json.str (toString name)),
      ("type", type.toJson),
      ("value", value.toJson),
      ("body", body.toJson)
    ]
  | .lit val => Json.mkObj [("type", "literal"), ("value", Json.str (toString val))]
  | .mdata data expr => Json.mkObj [
      ("type", "metadata"),
      ("data", Json.str (toString data)),
      ("expr", expr.toJson)
    ]
  | .proj name idx struct => Json.mkObj [
      ("type", "projection"),
      ("name", Json.str (toString name)),
      ("index", Json.num idx),
      ("struct", struct.toJson)
    ]

-- Export Expr as JSON string
def toJsonString (e : Expr) : String :=
  Json.pretty e.toJson

-- Batch export multiple expressions
def exportExprs (exprs : List Expr) : String :=
  let jsonExprs := exprs.map (λ e => e.toJson)
  Json.pretty (Json.arr jsonExprs)

end Lean.Expr

-- Command to export current environment expressions
#check Expr.toJson
#eval "Lean4 JSON export patch applied successfully"
                "# , # patch_description) ; lean4_patch } } . into () }
};
}
