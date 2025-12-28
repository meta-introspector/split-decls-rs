macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! Shared {
    () => {
        deps!();
        # [doc = " A pointer to an object protected by the epoch GC."] # [doc = ""] # [doc = " The pointer is valid for use only during the lifetime `'g`."] # [doc = ""] # [doc = " The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused"] # [doc = " least significant bits of the address."] pub struct Shared < 'g , T : 'g + ? Sized + Pointable > { data : * mut () , _marker : PhantomData < (& 'g () , * const T) > , }
    };
}

Shared!();