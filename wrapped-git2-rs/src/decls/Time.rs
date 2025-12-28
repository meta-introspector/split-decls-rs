macro_rules! Time {
    () => {
        # [doc = " Time in a signature"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Time { raw : raw :: git_time , }
    };
}

Time!()