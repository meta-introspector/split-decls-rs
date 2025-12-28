macro_rules! deps {
    () => {
        Any!();
        Field!();
        FieldValue!();
        Object!();
    };
}

macro_rules! FieldValueInner {
    () => {
        deps!();
        pub (crate) enum FieldValueInner < 'a > { # [doc = " Const value"] Value (Value) , # [doc = " Borrowed any value"] # [doc = " The first item is the [`std::any::type_name`] of the value used for"] # [doc = " debugging."] BorrowedAny (Cow < 'static , str > , & 'a (dyn Any + Send + Sync)) , # [doc = " Owned any value"] # [doc = " The first item is the [`std::any::type_name`] of the value used for"] # [doc = " debugging."] OwnedAny (Cow < 'static , str > , Box < dyn Any + Send + Sync >) , # [doc = " A list"] List (Vec < FieldValue < 'a > >) , # [doc = " A typed Field value"] WithType { # [doc = " Field value"] value : Box < FieldValue < 'a > > , # [doc = " Object name"] ty : Cow < 'static , str > , } , }
    };
}

FieldValueInner!();