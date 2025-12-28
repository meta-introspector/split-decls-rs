macro_rules! deps {
    () => {
        Parse!();
        ParseContext!();
        Result!();
        BareFunctionType!();
        SpecialName!();
        Encoding!();
        SubstitutionTable!();
        Name!();
        IndexStr!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Parse for Encoding { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Encoding , IndexStr < 'b >) > { try_begin_parse ! ("Encoding" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (Name :: parse (ctx , subs , input)) { if let Ok ((ty , tail)) = try_recurse ! (BareFunctionType :: parse (ctx , subs , tail)) { return Ok ((Encoding :: Function (name , ty) , tail)) ; } else { return Ok ((Encoding :: Data (name) , tail)) ; } } let (name , tail) = SpecialName :: parse (ctx , subs , input) ? ; Ok ((Encoding :: Special (name) , tail)) } }
    };
}

impl_69!()