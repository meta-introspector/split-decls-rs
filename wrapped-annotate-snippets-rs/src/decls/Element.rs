macro_rules! deps {
    () => {
        Message!();
        Patch!();
        Annotation!();
        Snippet!();
        Origin!();
        Group!();
        Padding!();
    };
}

macro_rules! Element {
    () => {
        deps!();
        # [doc = " A section of content within a [`Group`]"] # [derive (Clone , Debug)] # [non_exhaustive] pub enum Element < 'a > { Message (Message < 'a >) , Cause (Snippet < 'a , Annotation < 'a > >) , Suggestion (Snippet < 'a , Patch < 'a > >) , Origin (Origin < 'a >) , Padding (Padding) , }
    };
}

Element!();