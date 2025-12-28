macro_rules! deps {
    () => {
        Slice!();
        Pattern!();
        PatternElement!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 's , S : Slice < 's > > Pattern < S > { fn starts_on_new_line (& self) -> bool { ! self . has_leading_text_dot () && self . is_multiline () } fn is_multiline (& self) -> bool { self . elements . iter () . any (| elem | match elem { PatternElement :: TextElement { value } => value . as_ref () . contains ('\n') , PatternElement :: Placeable { expression } => is_select_expr (expression) , }) } fn has_leading_text_dot (& self) -> bool { if let Some (PatternElement :: TextElement { value }) = self . elements . first () { value . as_ref () . starts_with ('.') } else { false } } }
    };
}

impl_59!()