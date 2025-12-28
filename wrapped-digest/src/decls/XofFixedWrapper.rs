macro_rules! deps {
    () => {
        ExtendableOutput!();
    };
}

macro_rules! XofFixedWrapper {
    () => {
        deps!();
        # [doc = " Wrapper around [`ExtendableOutput`] types adding [`OutputSizeUser`] with the given size of `S`."] pub struct XofFixedWrapper < T : ExtendableOutput , S : ArraySize > { hash : T , size : PhantomData < S > , }
    };
}

XofFixedWrapper!();