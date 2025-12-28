macro_rules! deps {
    () => {
        DemangleWrite!();
        Demangle!();
        Result!();
        DemangleContext!();
    };
}

macro_rules! DemangleAsLeaf {
    () => {
        deps!();
        # [doc = " Demangle this thing in the leaf name position."] # [doc = ""] # [doc = " For most things this should be the same as its `Demangle`"] # [doc = " implementation. For `WellKnownComponent`s we need to strip the embedded"] # [doc = " `std::` namespace prefix."] pub (crate) trait DemangleAsLeaf < 'subs , W > where W : 'subs + DemangleWrite , { fn demangle_as_leaf < 'me , 'ctx > (& 'me self , ctx : & 'ctx mut DemangleContext < 'subs , W > ,) -> fmt :: Result ; }
    };
}

DemangleAsLeaf!();