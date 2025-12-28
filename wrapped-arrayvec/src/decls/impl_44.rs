macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [doc = " Create an `ArrayVec` from an array."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = ""] # [doc = " let mut array = ArrayVec::from([1, 2, 3]);"] # [doc = " assert_eq!(array.len(), 3);"] # [doc = " assert_eq!(array.capacity(), 3);"] # [doc = " ```"] impl < T , const CAP : usize > From < [T ; CAP] > for ArrayVec < T , CAP > { # [track_caller] fn from (array : [T ; CAP]) -> Self { let array = ManuallyDrop :: new (array) ; let mut vec = < ArrayVec < T , CAP > > :: new () ; unsafe { (& * array as * const [T ; CAP] as * const [MaybeUninit < T > ; CAP]) . copy_to_nonoverlapping (& mut vec . xs as * mut [MaybeUninit < T > ; CAP] , 1) ; vec . set_len (CAP) ; } vec } }
    };
}

impl_44!()