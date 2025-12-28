macro_rules! deps {
    () => {
        AixHeader!();
        Header!();
    };
}

macro_rules! MemberHeader {
    () => {
        deps!();
        # [doc = " An archive member header."] # [derive (Debug , Clone , Copy)] enum MemberHeader < 'data > { # [doc = " Common header used by many formats."] Common (& 'data archive :: Header) , # [doc = " AIX big archive header"] AixBig (& 'data archive :: AixHeader) , }
    };
}

MemberHeader!()