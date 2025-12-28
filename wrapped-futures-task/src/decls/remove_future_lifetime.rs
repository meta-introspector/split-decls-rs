macro_rules! remove_future_lifetime {
    () => {
        # [allow (single_use_lifetimes)] unsafe fn remove_future_lifetime < 'a , T > (ptr : * mut (dyn Future < Output = T > + 'a) ,) -> * mut (dyn Future < Output = T > + 'static) { unsafe { mem :: transmute (ptr) } }
    };
}

remove_future_lifetime!();