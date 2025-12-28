macro_rules! deps {
    () => {
        DynMap!();
    };
}

macro_rules! KeyMap {
    () => {
        deps!();
        # [repr (transparent)] pub struct KeyMap < KEY > { map : DynMap , _phantom : PhantomData < KEY > , }
    };
}

KeyMap!();