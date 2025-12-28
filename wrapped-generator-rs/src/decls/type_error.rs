macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! type_error {
    () => {
        deps!();
        # [inline] # [cold] fn type_error < A > (msg : & str) -> ! { error ! ("{msg}, expected type: {}" , std :: any :: type_name ::< A > ()) ; std :: panic :: panic_any (Error :: TypeErr) }
    };
}

type_error!()