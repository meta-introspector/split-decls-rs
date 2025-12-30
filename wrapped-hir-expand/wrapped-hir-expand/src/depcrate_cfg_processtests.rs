// Generated macro for tests (module)
macro_rules! Depcrate_cfg_processtests {
() => {
// Module: crate::cfg_process
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use cfg :: DnfExpr ; use expect_test :: { Expect , expect } ; use syntax :: { AstNode , SourceFile , ast :: Attr } ; use crate :: cfg_process :: parse_from_attr_token_tree ; fn check_dnf_from_syntax (input : & str , expect : Expect) { let parse = SourceFile :: parse (input , span :: Edition :: CURRENT) ; let node = match parse . tree () . syntax () . descendants () . find_map (Attr :: cast) { Some (it) => it , None => { let node = std :: any :: type_name :: < Attr > () ; panic ! ("Failed to make ast node `{node}` from text {input}") } } ; let node = node . clone_subtree () ; assert_eq ! (node . syntax () . text_range () . start () , 0 . into ()) ; let cfg = parse_from_attr_token_tree (& node . meta () . unwrap () . token_tree () . unwrap ()) . unwrap () ; let actual = format ! ("#![cfg({})]" , DnfExpr :: new (& cfg)) ; expect . assert_eq (& actual) ; } # [test] fn cfg_from_attr () { check_dnf_from_syntax (r#"#[cfg(test)]"# , expect ! [[r#"#![cfg(test)]"#]]) ; check_dnf_from_syntax (r#"#[cfg(not(never))]"# , expect ! [[r#"#![cfg(not(never))]"#]]) ; } }
};
}
