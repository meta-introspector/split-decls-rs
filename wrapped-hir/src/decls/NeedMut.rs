macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! NeedMut {
    () => {
        deps!();
        # [derive (Debug)] pub struct NeedMut { pub local : Local , pub span : InFile < SyntaxNodePtr > , }
    };
}

NeedMut!()