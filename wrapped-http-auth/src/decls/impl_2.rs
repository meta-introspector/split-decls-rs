macro_rules! deps {
    () => {
        Possibilities!();
        State!();
        ChallengeParser!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'i > ChallengeParser < 'i > { pub fn new (input : & 'i str) -> Self { ChallengeParser { input , pos : 0 , state : State :: PreToken { challenge : None , next : Possibilities (P_SCHEME) , } , } } }
    };
}

impl_2!()