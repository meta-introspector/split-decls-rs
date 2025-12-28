macro_rules! deps {
    () => {
        MetaItemListParser!();
        NameValueParser!();
    };
}

macro_rules! ArgParser {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [must_use] pub enum ArgParser < 'a > { NoArgs , List (MetaItemListParser < 'a >) , NameValue (NameValueParser) , }
    };
}

ArgParser!();