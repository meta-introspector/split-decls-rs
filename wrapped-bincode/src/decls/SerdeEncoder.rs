macro_rules! deps {
    () => {
        Encoder!();
    };
}

macro_rules! SerdeEncoder {
    () => {
        deps!();
        pub (super) struct SerdeEncoder < 'a , ENC : Encoder > { pub (super) enc : & 'a mut ENC , }
    };
}

SerdeEncoder!();