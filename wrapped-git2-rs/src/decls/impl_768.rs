macro_rules! deps {
    () => {
        SubmoduleUpdateOptions!();
        FetchOptions!();
        CheckoutBuilder!();
    };
}

macro_rules! impl_768 {
    () => {
        deps!();
        impl < 'cb > SubmoduleUpdateOptions < 'cb > { # [doc = " Return default options."] pub fn new () -> Self { SubmoduleUpdateOptions { checkout_builder : CheckoutBuilder :: new () , fetch_opts : FetchOptions :: new () , allow_fetch : true , } } unsafe fn raw (& mut self) -> raw :: git_submodule_update_options { let mut checkout_opts : raw :: git_checkout_options = mem :: zeroed () ; let init_res = raw :: git_checkout_init_options (& mut checkout_opts , raw :: GIT_CHECKOUT_OPTIONS_VERSION) ; assert_eq ! (0 , init_res) ; self . checkout_builder . configure (& mut checkout_opts) ; let opts = raw :: git_submodule_update_options { version : raw :: GIT_SUBMODULE_UPDATE_OPTIONS_VERSION , checkout_opts , fetch_opts : self . fetch_opts . raw () , allow_fetch : self . allow_fetch as c_int , } ; opts } # [doc = " Set checkout options."] pub fn checkout (& mut self , opts : CheckoutBuilder < 'cb >) -> & mut Self { self . checkout_builder = opts ; self } # [doc = " Set fetch options and allow fetching."] pub fn fetch (& mut self , opts : FetchOptions < 'cb >) -> & mut Self { self . fetch_opts = opts ; self . allow_fetch = true ; self } # [doc = " Allow or disallow fetching."] pub fn allow_fetch (& mut self , b : bool) -> & mut Self { self . allow_fetch = b ; self } }
    };
}

impl_768!();