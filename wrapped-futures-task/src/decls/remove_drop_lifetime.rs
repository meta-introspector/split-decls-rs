macro_rules! remove_drop_lifetime {
    () => {
        # [allow (single_use_lifetimes)] unsafe fn remove_drop_lifetime < 'a , T > (ptr : unsafe fn (* mut (dyn Future < Output = T > + 'a)) ,) -> unsafe fn (* mut (dyn Future < Output = T > + 'static)) { unsafe { mem :: transmute (ptr) } }
    };
}

remove_drop_lifetime!();