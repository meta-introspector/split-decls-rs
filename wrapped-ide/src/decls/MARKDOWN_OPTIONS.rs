macro_rules! MARKDOWN_OPTIONS {
    () => {
        const MARKDOWN_OPTIONS : Options = Options :: ENABLE_FOOTNOTES . union (Options :: ENABLE_TABLES) . union (Options :: ENABLE_TASKLISTS) ;
    };
}

MARKDOWN_OPTIONS!()