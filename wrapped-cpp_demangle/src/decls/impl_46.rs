macro_rules! deps {
    () => {
        DemangleWrite!();
        AutoDemangleContextInnerBarrier!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'ctx , 'a , W > Drop for AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { fn drop (& mut self) { if ! self . ctx . inner . is_empty () { log ! ("Context inner was not emptied, did demangling fail?") ; } mem :: swap (& mut self . saved_inner , & mut self . ctx . inner) ; } }
    };
}

impl_46!()