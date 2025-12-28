macro_rules! Mut {
    () => {
        # [repr (transparent)] pub struct Mut < 'a , T > where T : ? Sized , { pub ptr : NonNull < T > , lifetime : PhantomData < & 'a mut T > , }
    };
}

Mut!();