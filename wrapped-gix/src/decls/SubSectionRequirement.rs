macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! SubSectionRequirement {
    () => {
        deps!();
        # [doc = " Determine how subsections may be used with a given key, suitable for obtaining the full name for use in assignments."] # [derive (Debug , Copy , Clone)] pub enum SubSectionRequirement { # [doc = " Subsections must not be used, this key can only be below a section."] Never , # [doc = " The sub-section is used as parameter with the given name."] Parameter (& 'static str) , }
    };
}

SubSectionRequirement!()