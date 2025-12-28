macro_rules! deps {
    () => {
        DebugByte!();
        Transition!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl fmt :: Debug for Transition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use crate :: util :: escape :: DebugByte ; let Transition { start , end , next } = * self ; if self . start == self . end { write ! (f , "{:?} => {:?}" , DebugByte (start) , next . as_usize ()) } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , next . as_usize () ,) } } }
    };
}

impl_527!();