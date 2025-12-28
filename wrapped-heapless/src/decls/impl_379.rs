macro_rules! deps {
    () => {
        PeekMutInner!();
        Kind!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < T , K , S > PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { # [doc = " Removes the peeked value from the heap and returns it."] pub fn pop (mut this : Self) -> T { let value = this . heap . pop () . unwrap () ; this . sift = false ; value } }
    };
}

impl_379!()