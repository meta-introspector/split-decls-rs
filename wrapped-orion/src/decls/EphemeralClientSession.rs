macro_rules! deps {
    () => {
        PrivateKey!();
        PublicKey!();
    };
}

macro_rules! EphemeralClientSession {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] # [doc = " A key pair used to establish shared keys for a single session."] pub struct EphemeralClientSession { private_key : PrivateKey , public_key : PublicKey , }
    };
}

EphemeralClientSession!()