macro_rules! deps {
    () => {
        PacketlineReader!();
    };
}

macro_rules! ReadProcessOutputAndStatus {
    () => {
        deps!();
        struct ReadProcessOutputAndStatus < 'a > { inner : PacketlineReader < 'a > , }
    };
}

ReadProcessOutputAndStatus!()