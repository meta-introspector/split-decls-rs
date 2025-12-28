macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! AllConsuming {
    () => {
        deps!();
        # [doc = " Parser implementation for [all_consuming]"] pub struct AllConsuming < F > { parser : F , }
    };
}

AllConsuming!()