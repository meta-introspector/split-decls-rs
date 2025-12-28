macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! Weak {
    () => {
        deps!();
        # [doc = " `Weak` holds a non-owning reference to an object."] # [derive (Clone , PartialEq , Eq , Default)] pub struct Weak < I : Interface > (Option < imp :: IWeakReference > , PhantomData < I >) ;
    };
}

Weak!()