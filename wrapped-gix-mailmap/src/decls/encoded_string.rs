macro_rules! deps {
    () => {
        EncodedString!();
        EncodedStringRef!();
    };
}

macro_rules! encoded_string {
    () => {
        deps!();
        # [cfg (test)] mod encoded_string { use std :: cmp :: Ordering ; use crate :: snapshot :: util :: { EncodedString , EncodedStringRef } ; # [test] fn basic_ascii_case_folding () { assert_eq ! (EncodedString :: Utf8 ("FooBar" . into ()) . cmp_ref (EncodedStringRef :: Utf8 ("foobar")) , Ordering :: Equal) ; } # [test] fn no_advanced_unicode_folding () { assert_ne ! (EncodedString :: Utf8 ("Masse" . into ()) . cmp_ref (EncodedStringRef :: Utf8 ("Maße")) , Ordering :: Equal) ; } # [test] fn unknown_encoding_pairs_do_not_try_to_ignore_cases () { assert_ne ! (EncodedString :: Utf8 ("Foo" . into ()) . cmp_ref (EncodedStringRef :: Unknown ("foo" . into ())) , Ordering :: Equal) ; assert_ne ! (EncodedString :: Unknown ("Foo" . into ()) . cmp_ref (EncodedStringRef :: Utf8 ("foo")) , Ordering :: Equal) ; assert_ne ! (EncodedString :: Unknown ("Foo" . into ()) . cmp_ref (EncodedStringRef :: Unknown ("foo" . into ())) , Ordering :: Equal) ; } }
    };
}

encoded_string!()