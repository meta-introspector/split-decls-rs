macro_rules! IntrinsicUnstable {
    () => {
        # [doc = " A call to an intrinsic that is just not const-callable at all."] # [derive (Debug)] pub (crate) struct IntrinsicUnstable { pub name : Symbol , pub feature : Symbol , pub const_stable_indirect : bool , }
    };
}

IntrinsicUnstable!();