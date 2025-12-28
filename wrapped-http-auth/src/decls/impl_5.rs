macro_rules! deps {
    () => {
        ChallengeRef!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < 'i > ChallengeRef < 'i > { pub fn new (scheme : & 'i str) -> Self { ChallengeRef { scheme , params : Vec :: new () , } } }
    };
}

impl_5!()