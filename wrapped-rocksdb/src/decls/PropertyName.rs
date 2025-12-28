macro_rules! PropertyName {
    () => {
        # [doc = " An owned name of a RocksDB property."] # [doc = ""] # [doc = " The value is guaranteed to be a nul-terminated UTF-8 string. This means it"] # [doc = " can be converted to [`CString`] and [`String`] at zero cost."] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct PropertyName (CString) ;
    };
}

PropertyName!();