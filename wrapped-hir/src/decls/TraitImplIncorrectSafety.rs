macro_rules! deps {
    () => {
        Impl!();
    };
}

macro_rules! TraitImplIncorrectSafety {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TraitImplIncorrectSafety { pub file_id : HirFileId , pub impl_ : AstPtr < ast :: Impl > , pub should_be_safe : bool , }
    };
}

TraitImplIncorrectSafety!();