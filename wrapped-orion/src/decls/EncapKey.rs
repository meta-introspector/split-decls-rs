macro_rules! deps {
    () => {
        PkeParameters!();
        RingElementNTT!();
    };
}

macro_rules! EncapKey {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Clone)] # [doc = " ML-KEM encapsulation key."] pub (crate) struct EncapKey < const K : usize , const ENCODED_SIZE : usize , Pke : PkeParameters > { pub (crate) bytes : [u8 ; ENCODED_SIZE] , h_ek : [u8 ; SHA3_256_OUTSIZE] , t_hat : [RingElementNTT ; K] , mat_a : [[RingElementNTT ; K] ; K] , _phantom : PhantomData < Pke > , }
    };
}

EncapKey!()