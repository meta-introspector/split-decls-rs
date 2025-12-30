// Generated macro for Options (struct)
macro_rules! DepcrateOptions {
() => {
// Module: crate
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Configuration for the [`cmark_with_options()`] and [`cmark_resume_with_options()`] functions."] # [doc = " The defaults should provide decent spacing and most importantly, will"] # [doc = " provide a faithful rendering of your markdown document particularly when"] # [doc = " rendering it to HTML."] # [doc = ""] # [doc = " It's best used with its `Options::default()` implementation."] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Options < 'a > { pub newlines_after_headline : usize , pub newlines_after_paragraph : usize , pub newlines_after_codeblock : usize , pub newlines_after_htmlblock : usize , pub newlines_after_table : usize , pub newlines_after_rule : usize , pub newlines_after_list : usize , pub newlines_after_blockquote : usize , pub newlines_after_rest : usize , # [doc = " The amount of newlines placed after TOML or YAML metadata blocks at the beginning of a document."] pub newlines_after_metadata : usize , # [doc = " Token count for fenced code block. An appropriate value of this field can be decided by"] # [doc = " [`calculate_code_block_token_count()`]."] # [doc = " Note that the default value is `4` which allows for one level of nested code-blocks,"] # [doc = " which is typically a safe value for common kinds of markdown documents."] pub code_block_token_count : usize , pub code_block_token : char , pub list_token : char , pub ordered_list_token : char , pub increment_ordered_list_bullets : bool , pub emphasis_token : char , pub strong_token : & 'a str , }
};
}
