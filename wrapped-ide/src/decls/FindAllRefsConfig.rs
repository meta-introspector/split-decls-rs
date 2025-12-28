macro_rules! FindAllRefsConfig {
    () => {
        # [derive (Debug)] pub struct FindAllRefsConfig < 'a > { pub search_scope : Option < SearchScope > , pub minicore : MiniCore < 'a > , }
    };
}

FindAllRefsConfig!();