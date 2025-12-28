macro_rules! deps {
    () => {
        Error!();
        ObjectAccessor!();
        Result!();
        ValueAccessor!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < 'a > ObjectAccessor < 'a > { # [doc = " Return a reference to the value stored for `key`, if it is present,"] # [doc = " else `None`."] # [inline] pub fn get (& self , name : & str) -> Option < ValueAccessor < '_ > > { self . 0 . get (name) . map (ValueAccessor) } # [doc = " Like [`ObjectAccessor::get`], returns `Err` if the index does not exist"] # [inline] pub fn try_get (& self , name : & str) -> Result < ValueAccessor < '_ > > { self . 0 . get (name) . map (ValueAccessor) . ok_or_else (| | Error :: new (format ! ("internal: key \"{}\" not found" , name))) } # [doc = " Return an iterator over the key-value pairs of the object, in their"] # [doc = " order"] # [inline] pub fn iter (& self) -> impl Iterator < Item = (& Name , ValueAccessor < '_ >) > + '_ { self . 0 . iter () . map (| (name , value) | (name , ValueAccessor (value))) } # [doc = " Return an iterator over the keys of the object, in their order"] # [inline] pub fn keys (& self) -> impl Iterator < Item = & Name > + '_ { self . 0 . keys () } # [doc = " Return an iterator over the values of the object, in their order"] # [inline] pub fn values (& self) -> impl Iterator < Item = ValueAccessor < '_ > > + '_ { self . 0 . values () . map (ValueAccessor) } # [doc = " Returns the number of elements in the object"] # [inline] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Returns `true` if the object has no members"] # [must_use] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns a reference to the underlying IndexMap"] # [inline] pub fn as_index_map (& 'a self) -> & 'a IndexMap < Name , Value > { & self . 0 } }
    };
}

impl_515!()