macro_rules! deps {
    () => {
        Fold!();
        TokenId!();
        InlayHint!();
    };
}

macro_rules! StaticIndexedFile {
    () => {
        deps!();
        # [derive (Debug)] pub struct StaticIndexedFile { pub file_id : FileId , pub folds : Vec < Fold > , pub inlay_hints : Vec < InlayHint > , pub tokens : Vec < (TextRange , TokenId) > , }
    };
}

StaticIndexedFile!()