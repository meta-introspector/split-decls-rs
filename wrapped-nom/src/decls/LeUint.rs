macro_rules! LeUint {
    () => {
        # [doc = " Little endian unsigned integer parser"] struct LeUint < Uint , E > { bound : usize , e : PhantomData < E > , u : PhantomData < Uint > , }
    };
}

LeUint!()