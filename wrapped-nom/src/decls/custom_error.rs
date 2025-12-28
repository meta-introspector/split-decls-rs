macro_rules! deps {
    () => {
        CustomError!();
        IResult!();
    };
}

macro_rules! custom_error {
    () => {
        deps!();
        # [allow (dead_code)] fn custom_error (input : & [u8]) -> IResult < & [u8] , & [u8] , CustomError > { crate :: character :: streaming :: alphanumeric1 (input) }
    };
}

custom_error!()