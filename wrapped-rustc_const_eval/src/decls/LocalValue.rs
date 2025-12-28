macro_rules! deps {
    () => {
        Operand!();
    };
}

macro_rules! LocalValue {
    () => {
        deps!();
        # [doc = " Current value of a local variable"] # [doc = ""] # [doc = " This does not store the type of the local; the type is given by `body.local_decls` and can never"] # [doc = " change, so by not storing here we avoid having to maintain that as an invariant."] # [derive (Copy , Clone , Debug)] pub (super) enum LocalValue < Prov : Provenance = CtfeProvenance > { # [doc = " This local is not currently alive, and cannot be used at all."] Dead , # [doc = " A normal, live local."] # [doc = " Mostly for convenience, we re-use the `Operand` type here."] # [doc = " This is an optimization over just always having a pointer here;"] # [doc = " we can thus avoid doing an allocation when the local just stores"] # [doc = " immediate values *and* never has its address taken."] Live (Operand < Prov >) , }
    };
}

LocalValue!()