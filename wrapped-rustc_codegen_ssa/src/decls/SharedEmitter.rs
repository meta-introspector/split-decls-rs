macro_rules! deps {
    () => {
        SharedEmitterMessage!();
    };
}

macro_rules! SharedEmitter {
    () => {
        deps!();
        # [derive (Clone)] pub struct SharedEmitter { sender : Sender < SharedEmitterMessage > , }
    };
}

SharedEmitter!();