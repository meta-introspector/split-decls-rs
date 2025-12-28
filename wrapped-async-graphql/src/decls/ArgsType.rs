macro_rules! deps {
    () => {
        Directive!();
        Field!();
    };
}

macro_rules! ArgsType {
    () => {
        deps!();
        enum ArgsType < 'a > { Directive (& 'a str) , Field { field_name : & 'a str , type_name : & 'a str , } , }
    };
}

ArgsType!();