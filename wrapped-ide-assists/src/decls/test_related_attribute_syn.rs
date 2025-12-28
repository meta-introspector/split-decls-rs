macro_rules! test_related_attribute_syn {
    () => {
        # [doc = " This is a method with a heuristics to support test methods annotated with custom test annotations, such as"] # [doc = " `#[test_case(...)]`, `#[tokio::test]` and similar."] # [doc = " Also a regular `#[test]` annotation is supported."] # [doc = ""] # [doc = " It may produce false positives, for example, `#[wasm_bindgen_test]` requires a different command to run the test,"] # [doc = " but it's better than not to have the runnables for the tests at all."] pub fn test_related_attribute_syn (fn_def : & ast :: Fn) -> Option < ast :: Attr > { fn_def . attrs () . find_map (| attr | { let path = attr . path () ? ; let text = path . syntax () . text () . to_string () ; if text . starts_with ("test") || text . ends_with ("test") { Some (attr) } else { None } }) }
    };
}

test_related_attribute_syn!()