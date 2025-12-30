// Generated macro for CheckoutBuilder (struct)
macro_rules! Depcrate_buildCheckoutBuilder {
() => {
// Module: crate::build
// Provides: {"CheckoutBuilder"}
// Dependencies: {}
# [doc = " A builder struct for configuring checkouts of a repository."] pub struct CheckoutBuilder < 'cb > { their_label : Option < CString > , our_label : Option < CString > , ancestor_label : Option < CString > , target_dir : Option < CString > , paths : Vec < CString > , path_ptrs : Vec < * const c_char > , file_perm : Option < i32 > , dir_perm : Option < i32 > , disable_filters : bool , checkout_opts : u32 , progress : Option < Box < Progress < 'cb > > > , notify : Option < Box < Notify < 'cb > > > , notify_flags : CheckoutNotificationType , }
};
}
