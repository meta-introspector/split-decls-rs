macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Satisfy {
    () => {
        deps!();
        # [doc = " Parser implementation for [satisfy]"] pub struct Satisfy < F , MakeError > { predicate : F , make_error : MakeError , }
    };
}

Satisfy!()