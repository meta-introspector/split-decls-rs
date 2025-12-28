macro_rules! grammar {
    () => {
        # [doc = " Note: `include!` adds here a code generated from build.rs file."] # [doc = " In case feature `not-bootstrap-in-src` is:"] # [doc = " * OFF  -> include generated `grammar.rs` file from meta/src"] # [doc = " * ON   -> include generated `__pest_grammar.rs` file from target/build/..."] # [allow (missing_docs , unused_qualifications)] mod grammar { # [cfg (not (feature = "not-bootstrap-in-src"))] include ! ("grammar.rs") ; # [cfg (feature = "not-bootstrap-in-src")] include ! (concat ! (env ! ("OUT_DIR") , "/__pest_grammar.rs")) ; }
    };
}

grammar!()