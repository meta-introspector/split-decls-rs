macro_rules! IntrinsicNonConst {
    () => {
        # [doc = " A call to an intrinsic that is just not const-callable at all."] # [derive (Debug)] pub (crate) struct IntrinsicNonConst { pub name : Symbol , }
    };
}

IntrinsicNonConst!();