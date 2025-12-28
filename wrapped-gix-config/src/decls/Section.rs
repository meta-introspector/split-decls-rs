macro_rules! deps {
    () => {
        Event!();
        Header!();
    };
}

macro_rules! Section {
    () => {
        deps!();
        # [doc = " A parsed section containing the header and the section events, typically"] # [doc = " comprising the keys and their values."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Section < 'a > { # [doc = " The section name and subsection name, if any."] pub header : section :: Header < 'a > , # [doc = " The syntactic events found in this section."] pub events : Vec < Event < 'a > > , }
    };
}

Section!()