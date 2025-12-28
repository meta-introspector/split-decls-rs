macro_rules! deps {
    () => {
        DequeView!();
        Deque!();
    };
}

macro_rules! DequeInner {
    () => {
        deps!();
        # [doc = " Base struct for [`Deque`] and [`DequeView`], generic over the [`VecStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`Deque`] or [`DequeView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct DequeInner < T , S : VecStorage < T > + ? Sized > { phantom : PhantomData < T > , # [doc = " Front index. Always 0..=(N-1)"] front : usize , # [doc = " Back index. Always 0..=(N-1)."] back : usize , # [doc = " Used to distinguish \"empty\" and \"full\" cases when `front == back`."] # [doc = " May only be `true` if `front == back`, always `false` otherwise."] full : bool , buffer : S , }
    };
}

DequeInner!();