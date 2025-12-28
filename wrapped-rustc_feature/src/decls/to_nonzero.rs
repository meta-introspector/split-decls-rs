macro_rules! to_nonzero {
    () => {
        const fn to_nonzero (n : Option < u32 >) -> Option < NonZero < u32 > > { match n { None => None , Some (n) => NonZero :: new (n) , } }
    };
}

to_nonzero!()