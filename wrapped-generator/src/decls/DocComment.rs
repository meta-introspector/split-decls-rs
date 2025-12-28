macro_rules! DocComment {
    () => {
        # [doc = " Abstraction for the grammar and rule doc."] # [derive (Debug)] pub struct DocComment { # [doc = " The grammar documentation is defined at the beginning of a file with //!."] pub grammar_doc : String , # [doc = " HashMap for store all doc_comments for rules."] # [doc = " key is rule name, value is doc_comment."] pub line_docs : HashMap < String , String > , }
    };
}

DocComment!();