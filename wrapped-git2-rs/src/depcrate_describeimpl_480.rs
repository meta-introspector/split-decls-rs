// Generated macro for impl_480 (impl)
macro_rules! Depcrate_describeimpl_480 {
() => {
// Module: crate::describe
// Provides: {"impl_480"}
// Dependencies: {}
impl DescribeFormatOptions { # [doc = " Creates a new blank set of formatting options for a description."] pub fn new () -> DescribeFormatOptions { let mut opts = DescribeFormatOptions { raw : unsafe { mem :: zeroed () } , dirty_suffix : CString :: new (Vec :: new ()) . unwrap () , } ; opts . raw . version = 1 ; opts . raw . abbreviated_size = 7 ; opts } # [doc = " Sets the size of the abbreviated commit id to use."] # [doc = ""] # [doc = " The value is the lower bound for the length of the abbreviated string,"] # [doc = " and the default is 7."] pub fn abbreviated_size (& mut self , size : u32) -> & mut Self { self . raw . abbreviated_size = size as c_uint ; self } # [doc = " Sets whether or not the long format is used even when a shorter name"] # [doc = " could be used."] pub fn always_use_long_format (& mut self , long : bool) -> & mut Self { self . raw . always_use_long_format = long as c_int ; self } # [doc = " If the workdir is dirty and this is set, this string will be appended to"] # [doc = " the description string."] pub fn dirty_suffix (& mut self , suffix : & str) -> & mut Self { self . dirty_suffix = CString :: new (suffix) . unwrap () ; self . raw . dirty_suffix = self . dirty_suffix . as_ptr () ; self } }
};
}
