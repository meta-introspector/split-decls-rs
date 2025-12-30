// Generated macro for specify_simple (function)
macro_rules! Depcrate_config_testsspecify_simple {
() => {
// Module: crate::config::tests
// Provides: {"specify_simple"}
// Dependencies: {}
# [test] fn specify_simple () { let input_json = r##"[
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
                                "output-mode": "simple"
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
            ]"## ; let input_json = input_json . as_bytes () ; let (ctx , book) = mdbook :: preprocess :: CmdPreprocessor :: parse_input (input_json) . unwrap () ; let book = TestPreprocessor . run (& ctx , book) . unwrap () ; assert ! (book . iter () . any (| item | matches ! (item , BookItem :: PartTitle (title) if title == & format ! ("{:?}" , Mode :: Simple)))) }
};
}
