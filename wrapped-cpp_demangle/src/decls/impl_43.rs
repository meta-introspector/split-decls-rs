macro_rules! deps {
    () => {
        DemangleWrite!();
        AutoDemangleContextInnerBarrier!();
        DemangleContext!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 'ctx , 'a , W > AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { # [doc = " Set aside the current inner stack on the demangle context."] pub fn new (ctx : & 'ctx mut DemangleContext < 'a , W >) -> Self { let mut saved_inner = vec ! [] ; mem :: swap (& mut saved_inner , & mut ctx . inner) ; AutoDemangleContextInnerBarrier { ctx : ctx , saved_inner : saved_inner , } } }
    };
}

impl_43!();