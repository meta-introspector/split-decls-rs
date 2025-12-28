macro_rules! deps {
    () => {
        SubstitutionTable!();
        Expression!();
        SubobjectExpr!();
        IndexStr!();
        ParseContext!();
        Result!();
        Parse!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl Parse for SubobjectExpr { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SubobjectExpr , IndexStr < 'b >) > { try_begin_parse ! ("SubobjectExpr" , ctx , input) ; let (ty , tail) = TypeHandle :: parse (ctx , subs , input) ? ; let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; let (offset , tail) = parse_number (10 , true , tail) . unwrap_or ((0 , tail)) ; let tail = consume (b"E" , tail) ? ; Ok ((SubobjectExpr { ty : ty , expr : Box :: new (expr) , offset : offset , } , tail ,)) } }
    };
}

impl_305!();