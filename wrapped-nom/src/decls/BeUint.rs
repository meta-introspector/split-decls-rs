macro_rules! BeUint {
    () => {
        # [doc = " Big endian unsigned integer parser"] struct BeUint < Uint , E > { bound : usize , e : PhantomData < E > , u : PhantomData < Uint > , }
    };
}

BeUint!();