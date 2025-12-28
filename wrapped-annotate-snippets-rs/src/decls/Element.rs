macro_rules! deps {
    () => {
        Snippet!();
        Patch!();
        Padding!();
        Origin!();
        Annotation!();
        Group!();
        Message!();
    };
}

macro_rules! Element {
    () => {
        deps!();
        # [doc = " A section of content within a [`Group`]"] # [derive (Clone , Debug)] # [non_exhaustive] pub enum Element < 'a > { Message (Message < 'a >) , Cause (Snippet < 'a , Annotation < 'a > >) , Suggestion (Snippet < 'a , Patch < 'a > >) , Origin (Origin < 'a >) , Padding (Padding) , }
    };
}

Element!()