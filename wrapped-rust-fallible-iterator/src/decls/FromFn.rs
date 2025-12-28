macro_rules! FromFn {
    () => {
        # [doc = " An iterator using a function to generate new values."] # [derive (Clone , Debug)] pub struct FromFn < F > { fun : F , }
    };
}

FromFn!();