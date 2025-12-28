macro_rules! replace {
    () => {
        fn replace < 'a > (path : impl Into < Cow < 'a , BStr > > , find : u8 , replace : u8) -> Cow < 'a , BStr > { let path = path . into () ; match path { Cow :: Owned (mut path) => { for b in path . iter_mut () . filter (| b | * * b == find) { * b = replace ; } path . into () } Cow :: Borrowed (path) => { if ! path . contains (& find) { return path . into () ; } let mut path = path . to_owned () ; for b in path . iter_mut () . filter (| b | * * b == find) { * b = replace ; } path . into () } } }
    };
}

replace!();