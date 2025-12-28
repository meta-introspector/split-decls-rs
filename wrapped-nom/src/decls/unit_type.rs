macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! unit_type {
    () => {
        deps!();
        # [test] # [allow (deprecated)] fn unit_type () { assert_eq ! (tuple ::<&'static str , () , Error <&'static str >, () > (()) ("abxsbsh") , Ok (("abxsbsh" , ()))) ; assert_eq ! (tuple ::<&'static str , () , Error <&'static str >, () > (()) ("sdfjakdsas") , Ok (("sdfjakdsas" , ()))) ; assert_eq ! (tuple ::<&'static str , () , Error <&'static str >, () > (()) ("") , Ok (("" , ()))) ; }
    };
}

unit_type!();