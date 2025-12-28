macro_rules! LengthCoder {
    () => {
        pub (crate) struct LengthCoder { choice : [u16 ; 2] , low : [[u16 ; LOW_SYMBOLS] ; POS_STATES_MAX] , mid : [[u16 ; MID_SYMBOLS] ; POS_STATES_MAX] , high : [u16 ; HIGH_SYMBOLS] , }
    };
}

LengthCoder!()