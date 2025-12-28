macro_rules! deps {
    () => {
        InlayHint!();
        InlayHintPosition!();
        InlayHintLabel!();
        InlayKind!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl InlayHint { fn closing_paren_after (kind : InlayKind , range : TextRange) -> InlayHint { InlayHint { range , kind , label : InlayHintLabel :: from (")") , text_edit : None , position : InlayHintPosition :: After , pad_left : false , pad_right : false , resolve_parent : None , } } }
    };
}

impl_267!();