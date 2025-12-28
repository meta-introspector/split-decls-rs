macro_rules! deps {
    () => {
        InlineAsmOperand!();
        DefWithBody!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl InlineAsmOperand { pub fn parent (self , _db : & dyn HirDatabase) -> DefWithBody { self . owner . into () } pub fn name (& self , db : & dyn HirDatabase) -> Option < Name > { match & db . body (self . owner) [self . expr] { hir_def :: hir :: Expr :: InlineAsm (e) => e . operands . get (self . index) ? . 0 . clone () , _ => None , } } }
    };
}

impl_180!()