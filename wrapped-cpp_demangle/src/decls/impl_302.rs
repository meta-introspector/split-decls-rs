macro_rules! deps {
    () => {
        Result!();
        Error!();
        ParseContext!();
        Parse!();
        ResourceName!();
        IndexStr!();
        SubstitutionTable!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl Parse for ResourceName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ResourceName , IndexStr < 'b >) > { try_begin_parse ! ("ResourceName" , ctx , input) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } let mut end = input . as_ref () . iter () . map (| & c | c as char) . take_while (| & c | c != '$' || c . is_digit (36)) . count () ; if end == 0 { return Err (error :: Error :: UnexpectedText) ; } if input . range_from (end ..) . peek () == Some (b'$') { match input . range_from (end ..) . peek_second () { Some (b'S') | Some (b'_') | Some (b'$') => end += 2 , _ => return Err (error :: Error :: UnexpectedText) , } } let tail = input . range_from (end ..) ; let resource_name = ResourceName { start : input . index () , end : tail . index () , } ; Ok ((resource_name , tail)) } }
    };
}

impl_302!()