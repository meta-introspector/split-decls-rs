macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! Owned {
    () => {
        deps!();
        # [doc = " An owned heap-allocated object."] # [doc = ""] # [doc = " This type is very similar to `Box<T>`."] # [doc = ""] # [doc = " The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused"] # [doc = " least significant bits of the address."] pub struct Owned < T : ? Sized + Pointable > { data : * mut () , _marker : PhantomData < Box < T > > , }
    };
}

Owned!();