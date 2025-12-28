macro_rules! impl_from_trait {
    () => {
        # [doc = " Macro that implements the `From<[T]>` trait on a object called `$name`"] # [doc = " which has fields `value` and `original_length`. It implements From"] # [doc = " based on `$size` and this macro should, in most cases, only be used for"] # [doc = " types which have a fixed-length."] macro_rules ! impl_from_trait (($ name : ident , $ size : expr) => (impl From < [u8 ; $ size] > for $ name { # [inline] # [doc = " Make an object from a byte array."] fn from (bytes : [u8 ; $ size]) -> $ name { $ name { value : bytes , original_length : $ size } } })) ;
    };
}

impl_from_trait!()