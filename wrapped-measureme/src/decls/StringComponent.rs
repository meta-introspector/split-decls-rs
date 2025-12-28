macro_rules! deps {
    () => {
        StringId!();
    };
}

macro_rules! StringComponent {
    () => {
        deps!();
        # [doc = " A single component of a string. Used for building composite table entries."] pub enum StringComponent < 's > { Value (& 's str) , Ref (StringId) , }
    };
}

StringComponent!()