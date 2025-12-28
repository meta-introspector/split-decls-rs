macro_rules! deps {
    () => {
        Impl!();
    };
}

macro_rules! IncoherentImpl {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct IncoherentImpl { pub file_id : HirFileId , pub impl_ : AstPtr < ast :: Impl > , }
    };
}

IncoherentImpl!();