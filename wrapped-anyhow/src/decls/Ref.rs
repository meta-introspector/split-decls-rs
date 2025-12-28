macro_rules! Ref {
    () => {
        # [repr (transparent)] pub struct Ref < 'a , T > where T : ? Sized , { pub ptr : NonNull < T > , lifetime : PhantomData < & 'a T > , }
    };
}

Ref!();