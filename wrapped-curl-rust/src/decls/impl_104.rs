macro_rules! deps {
    () => {
        Iter!();
        Error!();
        List!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl List { # [doc = " Creates a new empty list of strings."] pub fn new () -> List { List { raw : ptr :: null_mut () , } } # [doc = " Appends some data into this list."] pub fn append (& mut self , data : & str) -> Result < () , Error > { let data = CString :: new (data) ? ; unsafe { let raw = curl_sys :: curl_slist_append (self . raw , data . as_ptr ()) ; assert ! (! raw . is_null ()) ; self . raw = raw ; Ok (()) } } # [doc = " Returns an iterator over the nodes in this list."] pub fn iter (& self) -> Iter < '_ > { Iter { _me : self , cur : self . raw , } } }
    };
}

impl_104!();