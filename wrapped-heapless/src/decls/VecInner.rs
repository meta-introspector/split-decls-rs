macro_rules! deps {
    () => {
        Vec!();
        LenType!();
        VecView!();
    };
}

macro_rules! VecInner {
    () => {
        deps!();
        # [doc = " Base struct for [`Vec`] and [`VecView`], generic over the [`VecStorage`]."] # [doc = ""] # [doc = " In most cases you should use [`Vec`] or [`VecView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "S: Zeroize"))] pub struct VecInner < T , LenT : LenType , S : VecStorage < T > + ? Sized > { phantom : PhantomData < T > , len : LenT , buffer : S , }
    };
}

VecInner!();