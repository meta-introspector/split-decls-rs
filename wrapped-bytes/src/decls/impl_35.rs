macro_rules! deps {
    () => {
        Limit!();
        BufMut!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > Limit < T > { # [doc = " Consumes this `Limit`, returning the underlying value."] pub fn into_inner (self) -> T { self . inner } # [doc = " Gets a reference to the underlying `BufMut`."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying `BufMut`."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Gets a mutable reference to the underlying `BufMut`."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying `BufMut`."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Returns the maximum number of bytes that can be written"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " If the inner `BufMut` has fewer bytes than indicated by this method then"] # [doc = " that is the actual number of available bytes."] pub fn limit (& self) -> usize { self . limit } # [doc = " Sets the maximum number of bytes that can be written."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " If the inner `BufMut` has fewer bytes than `lim` then that is the actual"] # [doc = " number of available bytes."] pub fn set_limit (& mut self , lim : usize) { self . limit = lim } }
    };
}

impl_35!()