macro_rules! deps {
    () => {
        DecodeError!();
        IntegerType!();
    };
}

macro_rules! invalid_varint_discriminant {
    () => {
        deps!();
        # [inline (never)] # [cold] const fn invalid_varint_discriminant < T > (expected : IntegerType , found : IntegerType ,) -> Result < T , DecodeError > { Err (DecodeError :: InvalidIntegerType { expected , found }) }
    };
}

invalid_varint_discriminant!()