macro_rules! deps {
    () => {
        TryGetError!();
    };
}

macro_rules! panic_advance {
    () => {
        deps!();
        # [doc = " Panic with a nice error message."] # [cold] fn panic_advance (error_info : & TryGetError) -> ! { panic ! ("advance out of bounds: the len is {} but advancing by {}" , error_info . available , error_info . requested) ; }
    };
}

panic_advance!();