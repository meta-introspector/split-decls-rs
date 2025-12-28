macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! Solution {
    () => {
        deps!();
        struct Solution < 'a , 'b > { text1 : Range < 'a > , text2 : Range < 'b > , diffs : Vec < Diff < 'a , 'b > > , }
    };
}

Solution!()