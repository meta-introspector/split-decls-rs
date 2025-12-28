macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'a > Entry < 'a > { # [doc = " The name to map to."] pub fn new_name (& self) -> Option < & 'a BStr > { self . new_name } # [doc = " The email map to."] pub fn new_email (& self) -> Option < & 'a BStr > { self . new_email } # [doc = " The name to look for and replace."] pub fn old_name (& self) -> Option < & 'a BStr > { self . old_name } # [doc = " The email to look for and replace."] pub fn old_email (& self) -> & 'a BStr { self . old_email } }
    };
}

impl_9!()