macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleContext!();
        DemangleAsInner!();
    };
}

macro_rules! AutoDemangleContextInnerBarrier {
    () => {
        deps!();
        # [doc (hidden)] # [derive (Debug)] pub struct AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { ctx : & 'ctx mut DemangleContext < 'a , W > , saved_inner : Vec < & 'a dyn DemangleAsInner < 'a , W > > , }
    };
}

AutoDemangleContextInnerBarrier!()