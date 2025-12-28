macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! SharedData {
    () => {
        deps!();
        # [doc = " Shared (SQLITE_SERIALIZE_NOCOPY) serialized database"] pub struct SharedData < 'conn > { phantom : PhantomData < & 'conn Connection > , ptr : NonNull < u8 > , sz : usize , }
    };
}

SharedData!();