macro_rules! deps {
    () => {
        Result!();
        ParametricBuiltinType!();
        BuiltinType!();
        ParseContext!();
        IndexStr!();
        SourceName!();
        SubstitutionTable!();
        Parse!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl Parse for BuiltinType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (BuiltinType , IndexStr < 'b >) > { try_begin_parse ! ("BuiltinType" , ctx , input) ; if let Ok ((ty , tail)) = try_recurse ! (StandardBuiltinType :: parse (ctx , subs , input)) { return Ok ((BuiltinType :: Standard (ty) , tail)) ; } if let Ok (tail) = consume (b"u" , input) { let (name , tail) = SourceName :: parse (ctx , subs , tail) ? ; Ok ((BuiltinType :: Extension (name) , tail)) } else { match try_recurse ! (ParametricBuiltinType :: parse (ctx , subs , input)) { Ok ((ty , tail)) => Ok ((BuiltinType :: Parametric (ty) , tail)) , Err (e) => Err (e) , } } } }
    };
}

impl_176!();