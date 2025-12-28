macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! ScopedInterface {
    () => {
        deps!();
        # [doc (hidden)] pub struct ScopedInterface < 'a , T : Interface > { interface : T , lifetime : PhantomData < & 'a T > , }
    };
}

ScopedInterface!()