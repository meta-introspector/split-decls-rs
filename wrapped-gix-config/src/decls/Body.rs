macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! Body {
    () => {
        deps!();
        # [doc = " A opaque type that represents a section body."] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Clone , Debug , Default)] pub struct Body < 'event > (pub (crate) Vec < Event < 'event > >) ;
    };
}

Body!()