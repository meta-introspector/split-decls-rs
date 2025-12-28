macro_rules! deps {
    () => {
        Task!();
        FuturesUnordered!();
    };
}

macro_rules! IterPinMut {
    () => {
        deps!();
        # [doc = " Mutable iterator over all futures in the unordered set."] # [derive (Debug)] pub struct IterPinMut < 'a , Fut > { pub (super) task : * const Task < Fut > , pub (super) len : usize , pub (super) _marker : PhantomData < & 'a mut FuturesUnordered < Fut > > , }
    };
}

IterPinMut!();