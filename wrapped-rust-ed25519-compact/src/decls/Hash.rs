macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! Hash {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct Hash { state : State , w : [u8 ; 128] , r : usize , len : usize , }
    };
}

Hash!();