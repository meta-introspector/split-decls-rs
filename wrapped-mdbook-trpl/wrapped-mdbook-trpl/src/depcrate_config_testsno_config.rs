// Generated macro for no_config (function)
macro_rules! Depcrate_config_testsno_config {
() => {
// Module: crate::config::tests
// Provides: {"no_config"}
// Dependencies: {}
# [test] fn no_config () { let input_json = r##"[
        {
            "root": "/path/to/book",
            "config": {
                "book": {
                    "authors": ["AUTHOR"],
                    "language": "en",
                    "multilingual": false,
                    "src": "src",
                    "title": "TITLE"
                },
                "preprocessor": {}
            },
            "renderer": "html",
            "mdbook_version": "0.4.21"
        },
        {
            "sections": [
                {
                    "Chapter": {
                        "name": "Chapter 1",
                        "content": "# Chapter 1\n",
                        "number": [1],
                        "sub_items": [],
                        "path": "chapter_1.md",
                        "source_path": "chapter_1.md",
                        "parent_names": []
                    }
                }
            ],
            "__non_exhaustive": null
        }
    ]"## ; let input_json = input_json . as_bytes () ; let (ctx , book) = mdbook :: preprocess :: CmdPreprocessor :: parse_input (input_json) . unwrap () ; let result = TestPreprocessor . run (& ctx , book) ; assert ! (result . is_err ()) ; let err = result . unwrap_err () ; assert_eq ! (format ! ("{err}") , "No config for 'test-preprocessor'") ; }
};
}
