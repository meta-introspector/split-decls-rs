macro_rules! Whitespace {
    () => {
        # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Debug)] struct Whitespace < 'a > { pre_key : Option < Cow < 'a , BStr > > , pre_sep : Option < Cow < 'a , BStr > > , post_sep : Option < Cow < 'a , BStr > > , }
    };
}

Whitespace!()