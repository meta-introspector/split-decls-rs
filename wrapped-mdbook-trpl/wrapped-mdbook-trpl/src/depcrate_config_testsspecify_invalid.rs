// Generated macro for specify_invalid (function)
macro_rules! Depcrate_config_testsspecify_invalid {
() => {
// Module: crate::config::tests
// Provides: {"specify_invalid"}
// Dependencies: {}
# [test] fn specify_invalid () { let input_json = r##"[
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
                        "preprocessor": {
                            "test-preprocessor": {
                                "output-mode": "nonsense"
                            }
                        }
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
            ]"## ; let input_json = input_json . as_bytes () ; let (ctx , book) = mdbook :: preprocess :: CmdPreprocessor :: parse_input (input_json) . unwrap () ; let result = TestPreprocessor . run (& ctx , book) . unwrap_err () ; assert_eq ! (format ! ("{result}") , "Bad config value '\"nonsense\"' for key 'output-mode'") ; }
};
}
