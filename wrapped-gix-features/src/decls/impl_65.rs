macro_rules! deps {
    () => {
        Reduce!();
        IdentityWithResult!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < Input , Error > Reduce for IdentityWithResult < Input , Error > { type Input = Result < Input , Self :: Error > ; type FeedProduce = Input ; type Output = () ; type Error = Error ; fn feed (& mut self , item : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { item } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
    };
}

impl_65!()