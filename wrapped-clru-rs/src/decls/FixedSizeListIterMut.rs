macro_rules! deps {
    () => {
        FixedSizeListNode!();
    };
}

macro_rules! FixedSizeListIterMut {
    () => {
        deps!();
        pub (crate) struct FixedSizeListIterMut < 'a , T > { ptr : NonNull < Option < FixedSizeListNode < T > > > , front : usize , back : usize , len : usize , _marker : std :: marker :: PhantomData < & 'a mut T > , }
    };
}

FixedSizeListIterMut!();