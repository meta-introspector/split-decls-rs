macro_rules! HighlightConfig {
    () => {
        # [derive (Copy , Clone , Debug)] pub struct HighlightConfig < 'a > { # [doc = " Whether to highlight strings"] pub strings : bool , # [doc = " Whether to highlight comments"] pub comments : bool , # [doc = " Whether to highlight punctuation"] pub punctuation : bool , # [doc = " Whether to specialize punctuation highlights"] pub specialize_punctuation : bool , # [doc = " Whether to highlight operator"] pub operator : bool , # [doc = " Whether to specialize operator highlights"] pub specialize_operator : bool , # [doc = " Whether to inject highlights into doc comments"] pub inject_doc_comment : bool , # [doc = " Whether to highlight the macro call bang"] pub macro_bang : bool , # [doc = " Whether to highlight unresolved things be their syntax"] pub syntactic_name_ref_highlighting : bool , pub minicore : MiniCore < 'a > , }
    };
}

HighlightConfig!();