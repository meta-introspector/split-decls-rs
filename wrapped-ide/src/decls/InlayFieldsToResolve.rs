macro_rules! InlayFieldsToResolve {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct InlayFieldsToResolve { pub resolve_text_edits : bool , pub resolve_hint_tooltip : bool , pub resolve_label_tooltip : bool , pub resolve_label_location : bool , pub resolve_label_command : bool , }
    };
}

InlayFieldsToResolve!()