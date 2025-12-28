macro_rules! AttrValue {
    () => {
        # [doc = " All possible states of an attribute."] # [doc = ""] # [doc = " This enum is used to interpret the value returned by"] # [doc = " [`Repository::get_attr`](crate::Repository::get_attr) and"] # [doc = " [`Repository::get_attr_bytes`](crate::Repository::get_attr_bytes)."] # [derive (Debug , Clone , Copy , Eq)] pub enum AttrValue < 'string > { # [doc = " The attribute is set to true."] True , # [doc = " The attribute is unset (set to false)."] False , # [doc = " The attribute is set to a [valid UTF-8 string](prim@str)."] String (& 'string str) , # [doc = " The attribute is set to a string that might not be [valid UTF-8](prim@str)."] Bytes (& 'string [u8]) , # [doc = " The attribute is not specified."] Unspecified , }
    };
}

AttrValue!()