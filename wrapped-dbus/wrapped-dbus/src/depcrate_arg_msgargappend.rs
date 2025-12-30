// Generated macro for Append (trait)
macro_rules! Depcrate_arg_msgargAppend {
() => {
// Module: crate::arg::msgarg
// Provides: {"Append"}
// Dependencies: {}
# [doc = " Types that can be appended to a message as arguments implement this trait."] pub trait Append { # [doc = " Performs the append operation by consuming self."] fn append (self , ia : & mut IterAppend) where Self : Sized { self . append_by_ref (ia) } # [doc = " Performs the append operation by borrowing self."] fn append_by_ref (& self , _ : & mut IterAppend) ; }
};
}
