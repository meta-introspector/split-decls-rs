macro_rules! deps {
    () => {
        ListAccessor!();
        Result!();
        Error!();
        ValueAccessor!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < 'a > ListAccessor < 'a > { # [doc = " Returns the number of elements in the list"] # [inline] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Returns `true` if the list has a length of 0"] # [inline] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Returns an iterator over the list"] # [inline] pub fn iter (& self) -> impl Iterator < Item = ValueAccessor < '_ > > + '_ { self . 0 . iter () . map (ValueAccessor) } # [doc = " Returns a reference to an element depending on the index"] # [inline] pub fn get (& self , idx : usize) -> Option < ValueAccessor < '_ > > { self . 0 . get (idx) . map (ValueAccessor) } # [doc = " Like [`ListAccessor::get`], returns `Err` if the index does not exist"] # [inline] pub fn try_get (& self , idx : usize) -> Result < ValueAccessor < '_ > > { self . get (idx) . ok_or_else (| | Error :: new (format ! ("internal: index \"{}\" not found" , idx))) } # [doc = " Returns a new ListAccessor that represents a slice of the original"] # [inline] pub fn as_slice (& self , start : usize , end : usize) -> Result < ListAccessor < 'a > > { if start <= end && end <= self . len () { Ok (ListAccessor (& self . 0 [start .. end])) } else { Err (Error :: new ("internal: invalid slice indices")) } } # [doc = " Returns a reference to the underlying `&[Value]`"] # [inline] pub fn as_values_slice (& self) -> & 'a [Value] { self . 0 } }
    };
}

impl_517!()