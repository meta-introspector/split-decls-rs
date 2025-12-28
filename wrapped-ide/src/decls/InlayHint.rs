macro_rules! deps {
    () => {
        InlayHintPosition!();
        InlayKind!();
        LazyProperty!();
        InlayHintLabel!();
    };
}

macro_rules! InlayHint {
    () => {
        deps!();
        # [derive (Debug , UpmapFromRaFixture)] pub struct InlayHint { # [doc = " The text range this inlay hint applies to."] pub range : TextRange , pub position : InlayHintPosition , pub pad_left : bool , pub pad_right : bool , # [doc = " The kind of this inlay hint."] pub kind : InlayKind , # [doc = " The actual label to show in the inlay hint."] pub label : InlayHintLabel , # [doc = " Text edit to apply when \"accepting\" this inlay hint."] pub text_edit : Option < LazyProperty < TextEdit > > , # [doc = " Range to recompute inlay hints when trying to resolve for this hint. If this is none, the"] # [doc = " hint does not support resolving."] pub resolve_parent : Option < TextRange > , }
    };
}

InlayHint!()