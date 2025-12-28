macro_rules! deps {
    () => {
        Directive!();
    };
}

macro_rules! enabled {
    () => {
        deps!();
        pub (crate) fn enabled (directives : & [Directive] , level : Level , target : & str) -> bool { for directive in directives . iter () . rev () { match directive . name { Some (ref name) if ! target . starts_with (& * * name) => { } Some (..) | None => return level <= directive . level , } } false }
    };
}

enabled!();