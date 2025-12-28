macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! TypeIndex {
    () => {
        deps!();
        pub struct TypeIndex { files : Vec < File > , types : HashMap < String , HashMap < String , Vec < (usize , usize) > > > , nested : HashMap < (usize , usize) , Vec < usize > > , }
    };
}

TypeIndex!()