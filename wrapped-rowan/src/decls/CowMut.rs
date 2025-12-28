macro_rules! CowMut {
    () => {
        # [derive (Debug)] pub (crate) enum CowMut < 'a , T > { Owned (T) , Borrowed (& 'a mut T) , }
    };
}

CowMut!()