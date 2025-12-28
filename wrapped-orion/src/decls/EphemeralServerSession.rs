macro_rules! deps {
    () => {
        PublicKey!();
        PrivateKey!();
    };
}

macro_rules! EphemeralServerSession {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] # [doc = " A key pair used to establish shared keys for a single session."] pub struct EphemeralServerSession { private_key : PrivateKey , public_key : PublicKey , }
    };
}

EphemeralServerSession!()