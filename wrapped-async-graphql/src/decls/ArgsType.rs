macro_rules! deps {
    () => {
        Field!();
        Directive!();
    };
}

macro_rules! ArgsType {
    () => {
        deps!();
        enum ArgsType < 'a > { Directive (& 'a str) , Field { field_name : & 'a str , type_name : & 'a str , } , }
    };
}

ArgsType!()