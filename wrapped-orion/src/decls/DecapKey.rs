macro_rules! deps {
    () => {
        RingElementNTT!();
        PkeParameters!();
    };
}

macro_rules! DecapKey {
    () => {
        deps!();
        pub (crate) struct DecapKey < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > { pub (crate) bytes : [u8 ; ENCODED_SIZE_DK] , s_hat : [RingElementNTT ; K] , _phantom : PhantomData < Pke > , }
    };
}

DecapKey!()