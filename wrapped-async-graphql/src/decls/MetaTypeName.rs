macro_rules! MetaTypeName {
    () => {
        # [derive (Clone , Copy , Eq , PartialEq , Debug)] pub enum MetaTypeName < 'a > { List (& 'a str) , NonNull (& 'a str) , Named (& 'a str) , }
    };
}

MetaTypeName!();