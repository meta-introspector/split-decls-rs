macro_rules! Version {
    () => {
        # [doc = " A version structure for making relative comparisons."] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct Version { major : usize , minor : usize , patch : usize , }
    };
}

Version!()