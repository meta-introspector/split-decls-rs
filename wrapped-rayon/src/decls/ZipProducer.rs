macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! ZipProducer {
    () => {
        deps!();
        struct ZipProducer < A : Producer , B : Producer > { a : A , b : B , }
    };
}

ZipProducer!()