macro_rules! deps {
    () => {
        Null!();
        Blob!();
    };
}

macro_rules! ValueRef {
    () => {
        deps!();
        # [doc = " A non-owning [dynamic type value](http://sqlite.org/datatype3.html). Typically, the"] # [doc = " memory backing this value is owned by SQLite."] # [doc = ""] # [doc = " See [`Value`](Value) for an owning dynamic type value."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum ValueRef < 'a > { # [doc = " The value is a `NULL` value."] Null , # [doc = " The value is a signed integer."] Integer (i64) , # [doc = " The value is a floating point number."] Real (f64) , # [doc = " The value is a text string."] Text (& 'a [u8]) , # [doc = " The value is a blob of data"] Blob (& 'a [u8]) , }
    };
}

ValueRef!()