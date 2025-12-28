macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! Suffix {
    () => {
        deps!();
        # [doc = " Integer suffixes that are supported by `git-config`."] # [doc = ""] # [doc = " These values are base-2 unit of measurements, not the base-10 variants."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] # [allow (missing_docs)] pub enum Suffix { Kibi , Mebi , Gibi , }
    };
}

Suffix!();