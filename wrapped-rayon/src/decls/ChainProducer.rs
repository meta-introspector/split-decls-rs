macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! ChainProducer {
    () => {
        deps!();
        struct ChainProducer < A , B > where A : Producer , B : Producer < Item = A :: Item > , { a_len : usize , a : A , b : B , }
    };
}

ChainProducer!();