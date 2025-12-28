macro_rules! deps {
    () => {
        Place!();
    };
}

macro_rules! Immediate {
    () => {
        deps!();
        # [doc = " An `Immediate` represents a single immediate self-contained Rust value."] # [doc = ""] # [doc = " For optimization of a few very common cases, there is also a representation for a pair of"] # [doc = " primitive values (`ScalarPair`). It allows Miri to avoid making allocations for checked binary"] # [doc = " operations and wide pointers. This idea was taken from rustc's codegen."] # [doc = " In particular, thanks to `ScalarPair`, arithmetic operations and casts can be entirely"] # [doc = " defined on `Immediate`, and do not have to work with a `Place`."] # [derive (Copy , Clone , Debug)] pub enum Immediate < Prov : Provenance = CtfeProvenance > { # [doc = " A single scalar value (must have *initialized* `Scalar` ABI)."] Scalar (Scalar < Prov >) , # [doc = " A pair of two scalar value (must have `ScalarPair` ABI where both fields are"] # [doc = " `Scalar::Initialized`)."] ScalarPair (Scalar < Prov > , Scalar < Prov >) , # [doc = " A value of fully uninitialized memory. Can have arbitrary size and layout, but must be sized."] Uninit , }
    };
}

Immediate!()