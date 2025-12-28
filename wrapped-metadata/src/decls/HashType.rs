macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! HashType {
    () => {
        deps!();
        type HashType < 'a > = HashMap < & 'a str , HashMap < & 'a str , Vec < Item < 'a > > > > ;
    };
}

HashType!();