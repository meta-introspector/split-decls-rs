macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        TokenTree!();
        TokenStream!();
        AttrTokenStream!();
        AttrTokenTree!();
        LazyAttrTokenStreamInner!();
    };
}

macro_rules! size_asserts {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (AttrTokenStream , 8) ; static_assert_size ! (AttrTokenTree , 32) ; static_assert_size ! (LazyAttrTokenStream , 8) ; static_assert_size ! (LazyAttrTokenStreamInner , 88) ; static_assert_size ! (Option < LazyAttrTokenStream >, 8) ; static_assert_size ! (TokenStream , 8) ; static_assert_size ! (TokenTree , 32) ; }
    };
}

size_asserts!();