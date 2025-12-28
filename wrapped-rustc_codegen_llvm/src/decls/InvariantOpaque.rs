macro_rules! InvariantOpaque {
    () => {
        # [repr (C)] struct InvariantOpaque < 'a > { _marker : PhantomData < & 'a mut & 'a () > , _opaque : Opaque , }
    };
}

InvariantOpaque!();