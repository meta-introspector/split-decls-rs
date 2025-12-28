macro_rules! Measure {
    () => {
        # [doc = " Associated data that can be used for measures (such as length)."] pub trait Measure : Debug + PartialOrd + Add < Self , Output = Self > + Default + Clone { }
    };
}

Measure!();