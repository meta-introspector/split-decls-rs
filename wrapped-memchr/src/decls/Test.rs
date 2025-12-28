macro_rules! Test {
    () => {
        # [doc = " A single substring test for forward and reverse searches."] # [derive (Clone , Debug)] struct Test { needle : String , haystack : String , fwd : Option < usize > , rev : Option < usize > , }
    };
}

Test!()