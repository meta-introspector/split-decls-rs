macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! IterDefinedNames {
    () => {
        deps!();
        # [doc = "\nAn iterator over all defined named flags.\n\nThis iterator will yield flags values for all defined named flags, regardless of\nwhether they are contained in a particular flags value.\n"] pub struct IterDefinedNames < B : 'static > { flags : & 'static [Flag < B >] , idx : usize , }
    };
}

IterDefinedNames!();