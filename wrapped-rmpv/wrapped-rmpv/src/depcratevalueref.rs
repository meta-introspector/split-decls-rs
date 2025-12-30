// Generated macro for ValueRef (enum)
macro_rules! DepcrateValueRef {
() => {
// Module: crate
// Provides: {"ValueRef"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub enum ValueRef < 'a > { # [doc = " Nil represents nil."] Nil , # [doc = " Boolean represents true or false."] Boolean (bool) , # [doc = " Integer represents an integer."] # [doc = ""] # [doc = " A value of an `Integer` object is limited from `-(2^63)` upto `(2^64)-1`."] Integer (Integer) , # [doc = " A 32-bit floating point number."] F32 (f32) , # [doc = " A 64-bit floating point number."] F64 (f64) , # [doc = " String extending Raw type represents a UTF-8 string."] String (Utf8StringRef < 'a >) , # [doc = " Binary extending Raw type represents a byte array."] Binary (& 'a [u8]) , # [doc = " Array represents a sequence of objects."] Array (Vec < ValueRef < 'a > >) , # [doc = " Map represents key-value pairs of objects."] Map (Vec < (ValueRef < 'a > , ValueRef < 'a >) >) , # [doc = " Extended implements Extension interface: represents a tuple of type information and a byte"] # [doc = " array where type information is an integer whose meaning is defined by applications."] Ext (i8 , & 'a [u8]) , }
};
}
