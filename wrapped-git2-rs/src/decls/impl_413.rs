macro_rules! deps {
    () => {
        Error!();
        Binding!();
        Mailmap!();
        Signature!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl Mailmap { # [doc = " Creates an empty, in-memory mailmap object."] pub fn new () -> Result < Mailmap , Error > { crate :: init () ; let mut ret = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_mailmap_new (& mut ret)) ; Ok (Binding :: from_raw (ret)) } } # [doc = " Creates an in-memory mailmap object representing the given buffer."] pub fn from_buffer (buf : & str) -> Result < Mailmap , Error > { crate :: init () ; let mut ret = ptr :: null_mut () ; let len = buf . len () ; let buf = CString :: new (buf) ? ; unsafe { try_call ! (raw :: git_mailmap_from_buffer (& mut ret , buf , len)) ; Ok (Binding :: from_raw (ret)) } } # [doc = " Adds a new entry to this in-memory mailmap object."] pub fn add_entry (& mut self , real_name : Option < & str > , real_email : Option < & str > , replace_name : Option < & str > , replace_email : & str ,) -> Result < () , Error > { let real_name = crate :: opt_cstr (real_name) ? ; let real_email = crate :: opt_cstr (real_email) ? ; let replace_name = crate :: opt_cstr (replace_name) ? ; let replace_email = CString :: new (replace_email) ? ; unsafe { try_call ! (raw :: git_mailmap_add_entry (self . raw , real_name , real_email , replace_name , replace_email)) ; Ok (()) } } # [doc = " Resolves a signature to its real name and email address."] pub fn resolve_signature (& self , sig : & Signature < '_ >) -> Result < Signature < 'static > , Error > { let mut ret = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_mailmap_resolve_signature (& mut ret , &* self . raw , sig . raw ())) ; Ok (Binding :: from_raw (ret)) } } }
    };
}

impl_413!();