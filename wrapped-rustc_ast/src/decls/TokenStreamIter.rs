macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! TokenStreamIter {
    () => {
        deps!();
        # [derive (Clone)] pub struct TokenStreamIter < 't > { stream : & 't TokenStream , index : usize , }
    };
}

TokenStreamIter!()