macro_rules! NoPretty {
    () => {
        # [doc = " Avoid \"pretty\" debug"] pub struct NoPretty < T > (pub T) ;
    };
}

NoPretty!()