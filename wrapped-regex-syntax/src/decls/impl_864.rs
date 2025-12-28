macro_rules! deps {
    () => {
        Result!();
        Utf8Sequence!();
        Formatter!();
    };
}

macro_rules! impl_864 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Sequence { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: Utf8Sequence :: * ; match * self { One (ref r) => write ! (f , "{r:?}") , Two (ref r) => write ! (f , "{:?}{:?}" , r [0] , r [1]) , Three (ref r) => write ! (f , "{:?}{:?}{:?}" , r [0] , r [1] , r [2]) , Four (ref r) => { write ! (f , "{:?}{:?}{:?}{:?}" , r [0] , r [1] , r [2] , r [3]) } } } }
    };
}

impl_864!()