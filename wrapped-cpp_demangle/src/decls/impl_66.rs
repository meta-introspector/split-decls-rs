macro_rules! deps {
    () => {
        Parse!();
        GlobalCtorDtor!();
        Encoding!();
        Type!();
        Result!();
        SubstitutionTable!();
        ParseContext!();
        MangledName!();
        IndexStr!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Parse for MangledName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (MangledName , IndexStr < 'b >) > { try_begin_parse ! ("MangledName" , ctx , input) ; if let Ok (tail) = consume (b"_Z" , input) . or_else (| _ | consume (b"__Z" , input)) { let (encoding , tail) = Encoding :: parse (ctx , subs , tail) ? ; let (clone_suffixes , tail) = zero_or_more (ctx , subs , tail) ? ; return Ok ((MangledName :: Encoding (encoding , clone_suffixes) , tail)) ; } if let Ok (tail) = consume (b"___Z" , input) . or_else (| _ | consume (b"____Z" , input)) { let (encoding , tail) = Encoding :: parse (ctx , subs , tail) ? ; let tail = consume (b"_block_invoke" , tail) ? ; let tail_opt = match consume (b"_" , tail) . or_else (| _ | consume (b"." , tail)) { Ok (tail) => Some (parse_number (10 , false , tail) ?) , Err (_) => parse_number (10 , false , tail) . ok () , } ; let (digits , tail) = match tail_opt { Some ((digits , tail)) => (Some (digits) , tail) , None => (None , tail) , } ; return Ok ((MangledName :: BlockInvoke (encoding , digits) , tail)) ; } if let Ok (tail) = consume (b"_GLOBAL_" , input) { let (global_ctor_dtor , tail) = GlobalCtorDtor :: parse (ctx , subs , tail) ? ; return Ok ((MangledName :: GlobalCtorDtor (global_ctor_dtor) , tail)) ; } let (ty , tail) = TypeHandle :: parse (ctx , subs , input) ? ; Ok ((MangledName :: Type (ty) , tail)) } }
    };
}

impl_66!();