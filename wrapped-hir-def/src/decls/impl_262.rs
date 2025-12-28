macro_rules! deps {
    () => {
        HygieneId!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl HygieneId { pub const ROOT : Self = Self (span :: SyntaxContext :: root (Edition :: Edition2015)) ; pub fn new (mut ctx : span :: SyntaxContext) -> Self { ctx . remove_root_edition () ; Self (ctx) } pub (crate) fn lookup (self) -> SyntaxContext { self . 0 } pub (crate) fn is_root (self) -> bool { self . 0 . is_root () } }
    };
}

impl_262!()