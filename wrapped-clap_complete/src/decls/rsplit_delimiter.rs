macro_rules! rsplit_delimiter {
    () => {
        fn rsplit_delimiter < 's , 'o > (value : Result < & 's str , & 'o OsStr > , delimiter : Option < char > ,) -> Option < (Option < & 's str > , Result < & 's str , & 'o OsStr >) > { let delimiter = delimiter ? ; let value = value . ok () ? ; let pos = value . rfind (delimiter) ? ; let (prefix , value) = value . split_at (pos + delimiter . len_utf8 ()) ; Some ((Some (prefix) , Ok (value))) }
    };
}

rsplit_delimiter!();