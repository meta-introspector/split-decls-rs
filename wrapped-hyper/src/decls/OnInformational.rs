macro_rules! deps {
    () => {
        UserDataPointer!();
    };
}

macro_rules! OnInformational {
    () => {
        deps!();
        # [derive (Clone)] struct OnInformational { func : hyper_request_on_informational_callback , data : UserDataPointer , }
    };
}

OnInformational!()