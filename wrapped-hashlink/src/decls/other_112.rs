macro_rules! deps {
    () => {
        ValueLinks!();
        FreeLink!();
    };
}

macro_rules! other_112 {
    () => {
        deps!();
        union Links < K , V > { value : ValueLinks < K , V > , free : FreeLink < K , V > , }
    };
}

other_112!()