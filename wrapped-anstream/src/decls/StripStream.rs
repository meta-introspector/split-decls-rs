macro_rules! deps {
    () => {
        StripBytes!();
    };
}

macro_rules! StripStream {
    () => {
        deps!();
        # [doc = " Only pass printable data to the inner `Write`"] # [derive (Debug)] pub struct StripStream < S > where S : std :: io :: Write , { raw : S , state : StripBytes , }
    };
}

StripStream!();