macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! MakeComplete {
    () => {
        deps!();
        # [doc = " Parser implementation for [complete]"] pub struct MakeComplete < F > { parser : F , }
    };
}

MakeComplete!();