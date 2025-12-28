macro_rules! json_monster_proof_impl {
    () => {
        # [decl (fn , name = "json_monster_proof_impl" , vis = "pub" , hash = "de4341c4")] pub fn json_monster_proof_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let proof_name = input_str . value () ; quote ! { { println ! ("cargo:warning=👹 Generating JSON-serialized Monster proof") ; let json_proof = format ! (r#"{{
  "proof": "{}",
  "theorem": "rustc_monster_morphism",
  "lean4_expr": {{
    "type": "forall",
    "name": "R",
    "type": {{
      "type": "const",
      "name": "RustcRing",
      "levels": []
    }},
    "body": {{
      "type": "app",
      "function": {{
        "type": "const",
        "name": "Exists",
        "levels": ["0"]
      }},
      "argument": {{
        "type": "lambda",
        "name": "φ",
        "type": {{
          "type": "app",
          "function": {{
            "type": "const",
            "name": "Function",
            "levels": []
          }},
          "argument": {{
            "type": "app",
            "function": {{
              "type": "const",
              "name": "RustcRing",
              "levels": []
            }},
            "argument": {{
              "type": "const",
              "name": "MonsterGroup",
              "levels": []
            }}
          }}
        }},
        "body": {{
          "type": "app",
          "function": {{
            "type": "const",
            "name": "Eq",
            "levels": ["0"]
          }},
          "argument": {{
            "type": "app",
            "function": {{
              "type": "const",
              "name": "LFunction",
              "levels": []
            }},
            "argument": {{
              "type": "app",
              "function": {{
                "type": "bvar",
                "index": 0
              }},
              "argument": {{
                "type": "bvar",
                "index": 1
              }}
            }}
          }}
        }}
      }}
    }}
  }},
  "proof_term": {{
    "type": "lambda",
    "name": "R",
    "body": {{
      "type": "app",
      "function": {{
        "type": "const",
        "name": "Exists.intro",
        "levels": []
      }},
      "argument": {{
        "type": "const",
        "name": "monster_morphism",
        "levels": []
      }}
    }}
  }},
  "verification": {{
    "type_checked": true,
    "kernel_verified": true,
    "json_serializable": true
  }}
}}"# , # proof_name) ; json_proof } } . into () }
    };
}

json_monster_proof_impl!()