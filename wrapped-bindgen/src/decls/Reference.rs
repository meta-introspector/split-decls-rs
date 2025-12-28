macro_rules! deps {
    () => {
        Filter!();
        ReferenceStyle!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [derive (Debug)] pub struct Reference { pub name : String , pub filter : Filter , pub style : ReferenceStyle , }
    };
}

Reference!()