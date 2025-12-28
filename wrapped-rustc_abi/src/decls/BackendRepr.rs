macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! BackendRepr {
    () => {
        deps!();
        # [doc = " The way we represent values to the backend"] # [doc = ""] # [doc = " Previously this was conflated with the \"ABI\" a type is given, as in the platform-specific ABI."] # [doc = " In reality, this implies little about that, but is mostly used to describe the syntactic form"] # [doc = " emitted for the backend, as most backends handle SSA values and blobs of memory differently."] # [doc = " The psABI may need consideration in doing so, but this enum does not constitute a promise for"] # [doc = " how the value will be lowered to the calling convention, in itself."] # [doc = ""] # [doc = " Generally, a codegen backend will prefer to handle smaller values as a scalar or short vector,"] # [doc = " and larger values will usually prefer to be represented as memory."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum BackendRepr { Scalar (Scalar) , ScalarPair (Scalar , Scalar) , SimdVector { element : Scalar , count : u64 , } , Memory { # [doc = " If true, the size is exact, otherwise it's only a lower bound."] sized : bool , } , }
    };
}

BackendRepr!()