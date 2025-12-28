macro_rules! Map {
    () => {
        # [doc = " An adaptor to provide access to a part of larger structure."] # [doc = ""] # [doc = " This is the *active* part of this module. Use the [module documentation](index.html) for the"] # [doc = " details."] # [derive (Copy , Clone , Debug)] pub struct Map < A , T , F > { access : A , projection : F , _t : PhantomData < fn () -> T > , }
    };
}

Map!()