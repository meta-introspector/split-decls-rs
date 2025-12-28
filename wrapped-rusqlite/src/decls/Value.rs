macro_rules! deps {
    () => {
        Null!();
        Blob!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [doc = " Owning [dynamic type value](http://sqlite.org/datatype3.html). Value's type is typically"] # [doc = " dictated by SQLite (not by the caller)."] # [doc = ""] # [doc = " See [`ValueRef`](crate::types::ValueRef) for a non-owning dynamic type"] # [doc = " value."] # [derive (Clone , Debug , PartialEq)] pub enum Value { # [doc = " The value is a `NULL` value."] Null , # [doc = " The value is a signed integer."] Integer (i64) , # [doc = " The value is a floating point number."] Real (f64) , # [doc = " The value is a text string."] Text (String) , # [doc = " The value is a blob of data"] Blob (Vec < u8 >) , }
    };
}

Value!();