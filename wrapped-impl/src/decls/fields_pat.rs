macro_rules! deps {
    () => {
        Field!();
        MemberUnraw!();
    };
}

macro_rules! fields_pat {
    () => {
        deps!();
        fn fields_pat (fields : & [Field]) -> TokenStream { let mut members = fields . iter () . map (| field | & field . member) . peekable () ; match members . peek () { Some (MemberUnraw :: Named (_)) => quote ! ({ # (# members) ,* }) , Some (MemberUnraw :: Unnamed (_)) => { let vars = members . map (| member | match member { MemberUnraw :: Unnamed (index) => format_ident ! ("_{}" , index) , MemberUnraw :: Named (_) => unreachable ! () , }) ; quote ! ((# (# vars) ,*)) } None => quote ! ({ }) , } }
    };
}

fields_pat!()