macro_rules! deps {
    () => {
        SerdeEncoder!();
    };
}

macro_rules! Compound {
    () => {
        deps!();
        type Compound < 'a , ENC > = SerdeEncoder < 'a , ENC > ;
    };
}

Compound!()