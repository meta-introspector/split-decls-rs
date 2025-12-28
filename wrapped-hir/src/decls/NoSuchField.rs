macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! NoSuchField {
    () => {
        deps!();
        # [derive (Debug)] pub struct NoSuchField { pub field : InFile < AstPtr < Either < ast :: RecordExprField , ast :: RecordPatField > > > , pub private : Option < Field > , pub variant : VariantId , }
    };
}

NoSuchField!()