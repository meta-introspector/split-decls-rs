macro_rules! CompleteSemicolon {
    () => {
        # [derive (Debug)] pub (crate) enum CompleteSemicolon { DoNotComplete , CompleteSemi , CompleteComma , }
    };
}

CompleteSemicolon!()