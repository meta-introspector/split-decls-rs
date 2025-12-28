macro_rules! deps {
    () => {
        SharedEmitterMessage!();
    };
}

macro_rules! SharedEmitterMain {
    () => {
        deps!();
        pub struct SharedEmitterMain { receiver : Receiver < SharedEmitterMessage > , }
    };
}

SharedEmitterMain!();