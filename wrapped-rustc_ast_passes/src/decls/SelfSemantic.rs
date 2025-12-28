macro_rules! SelfSemantic {
    () => {
        # [doc = " Is `self` allowed semantically as the first parameter in an `FnDecl`?"] enum SelfSemantic { Yes , No , }
    };
}

SelfSemantic!()