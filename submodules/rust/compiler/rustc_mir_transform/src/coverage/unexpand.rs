mkuse!{use rustc_span :: Span ;}

macro_rules! unexpand_into_body_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unexpand_into_body_span in module {}", module_path!());
    };
}

mkfn!{
    unexpand_into_body_span_introspect!();
    # [doc = " Walks through the expansion ancestors of `original_span` to find a span that"] # [doc = " is contained in `body_span` and has the same [syntax context] as `body_span`."] pub (crate) fn unexpand_into_body_span (original_span : Span , body_span : Span) -> Option < Span > { original_span . find_ancestor_inside_same_ctxt (body_span) }
}