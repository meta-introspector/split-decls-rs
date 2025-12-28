macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! arbitrary_take_rest {
    () => {
        deps!();
        # [test] fn arbitrary_take_rest () { let x = [1 , 2 , 3 , 4] ; assert_eq ! (checked_arbitrary_take_rest ::<& [u8] > (Unstructured :: new (& x)) . unwrap () , & [1 , 2 , 3 , 4]) ; assert_eq ! (checked_arbitrary_take_rest ::< Vec < u8 >> (Unstructured :: new (& x)) . unwrap () , & [2 , 4]) ; assert_eq ! (&* checked_arbitrary_take_rest ::< Box < [u8] >> (Unstructured :: new (& x)) . unwrap () , & [2 , 4]) ; assert_eq ! (&* checked_arbitrary_take_rest ::< Arc < [u8] >> (Unstructured :: new (& x)) . unwrap () , & [2 , 4]) ; assert_eq ! (&* checked_arbitrary_take_rest ::< Rc < [u8] >> (Unstructured :: new (& x)) . unwrap () , & [2 , 4]) ; assert_eq ! (checked_arbitrary_take_rest ::< Vec < u32 >> (Unstructured :: new (& x)) . unwrap () , & [0x040302]) ; assert_eq ! (checked_arbitrary_take_rest ::< String > (Unstructured :: new (& x)) . unwrap () , "\x01\x02\x03\x04") ; assert_eq ! (checked_arbitrary_take_rest ::<& [u8] > (Unstructured :: new (& [])) . unwrap () , & []) ; assert_eq ! (checked_arbitrary_take_rest ::< Vec < u8 >> (Unstructured :: new (& [])) . unwrap () , & []) ; assert_eq ! (checked_arbitrary_take_rest ::< String > (Unstructured :: new (& [1 , 0xFF , 2])) . unwrap () , "\x01") ; }
    };
}

arbitrary_take_rest!()